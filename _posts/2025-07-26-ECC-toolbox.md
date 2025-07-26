---
layout: splash
title: Elliptic Curve Cryptography Toolbox.
permalink: /tools/ecc
---

## EC Cryptography Toolbox

This tool calculates the public key from a private key using elliptic curve cryptography with the NIST P-256 curve.

<div class="notice--info">
<strong>Info:</strong> Enter your private key as a hexadecimal string (64 characters for P-256). The tool will calculate the corresponding public key coordinates.
</div>

<form id="crypto-form">
  <div class="form-group">
    <label for="curve"><strong>Elliptic Curve:</strong></label>
    <select id="curve" class="form-control">
      <option value="P-256">NIST P-256 (secp256r1)</option>
    </select>
  </div>

  <div class="form-group">
    <label for="privateKey"><strong>Private Key A (Hex):</strong></label>
    <input type="text" id="privateKey" class="form-control" 
           placeholder="Enter 64-character hexadecimal private key for Party A"
           maxlength="64" style="font-family: monospace;">
    <small class="form-text text-muted">Example: c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5</small>
  </div>

  <div class="form-group">
    <label for="privateKeyB"><strong>Private Key B (Hex):</strong></label>
    <input type="text" id="privateKeyB" class="form-control" 
           placeholder="Enter 64-character hexadecimal private key for Party B"
           maxlength="64" style="font-family: monospace;">
    <small class="form-text text-muted">Example: f8f8a2f43c8376ccb0871305060d7b27b0554d2cc72bccf41b2705608452f315</small>
  </div>

  <div class="text-center">
    <button type="button" onclick="loadSampleKeys()" class="btn btn--info btn--small">Load Sample Keys</button>
    <button type="button" onclick="calculatePublicKey()" class="btn btn--primary">Calculate Public Keys</button>
    <button type="button" onclick="performKeyExchange()" class="btn btn--success">🔄 Key Exchange (ECDH)</button>
  </div>
</form>

<div id="results" style="margin-top: 2rem;">
  <h3>Public Keys (Uncompressed Format)</h3>
  
  <div class="row">
    <div class="col-md-6">
      <h4>Party A Public Key</h4>
      <div class="form-group">
        <label for="publicKeyAX"><strong>X Coordinate:</strong></label>
        <textarea id="publicKeyAX" class="form-control" readonly rows="2" 
                  placeholder="Party A X coordinate..." 
                  style="font-family: monospace; font-size: 0.9rem;"></textarea>
      </div>
      <div class="form-group">
        <label for="publicKeyAY"><strong>Y Coordinate:</strong></label>
        <textarea id="publicKeyAY" class="form-control" readonly rows="2" 
                  placeholder="Party A Y coordinate..." 
                  style="font-family: monospace; font-size: 0.9rem;"></textarea>
      </div>
    </div>
    
    <div class="col-md-6">
      <h4>Party B Public Key</h4>
      <div class="form-group">
        <label for="publicKeyBX"><strong>X Coordinate:</strong></label>
        <textarea id="publicKeyBX" class="form-control" readonly rows="2" 
                  placeholder="Party B X coordinate..." 
                  style="font-family: monospace; font-size: 0.9rem;"></textarea>
      </div>
      <div class="form-group">
        <label for="publicKeyBY"><strong>Y Coordinate:</strong></label>
        <textarea id="publicKeyBY" class="form-control" readonly rows="2" 
                  placeholder="Party B Y coordinate..." 
                  style="font-family: monospace; font-size: 0.9rem;"></textarea>
      </div>
    </div>
  </div>
  
  <div style="margin-top: 2rem;">
    <h3>🔐 ECDH Shared Secret</h3>
    <div class="form-group">
      <label for="sharedSecret"><strong>Shared Secret (X Coordinate):</strong></label>
      <textarea id="sharedSecret" class="form-control" readonly rows="2" 
                placeholder="Shared secret will appear here after key exchange..." 
                style="font-family: monospace; font-size: 0.9rem; background-color: #f8f9fa;"></textarea>
      <small class="form-text text-muted">In ECDH, only the X coordinate of the shared point is typically used as the shared secret.</small>
    </div>
  </div>
</div>

<div id="error"></div>

<script>
// NIST P-256 curve parameters
const P256 = {
    // Prime field
    p: BigInt('0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff'),
    // Curve parameter a
    a: BigInt('0xffffffff00000001000000000000000000000000fffffffffffffffffffffffc'),
    // Curve parameter b  
    b: BigInt('0x5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b'),
    // Order of the base point
    n: BigInt('0xffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551'),
    // Base point coordinates
    Gx: BigInt('0x6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296'),
    Gy: BigInt('0x4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5')
};

// Modular arithmetic functions
function mod(a, m) {
    return ((a % m) + m) % m;
}

function modInverse(a, m) {
    if (a < 0n) a = mod(a, m);
    
    let [old_r, r] = [a, m];
    let [old_s, s] = [1n, 0n];
    
    while (r !== 0n) {
        const quotient = old_r / r;
        [old_r, r] = [r, old_r - quotient * r];
        [old_s, s] = [s, old_s - quotient * s];
    }
    
    return mod(old_s, m);
}

// Point addition on elliptic curve
function pointAdd(p1, p2) {
    if (!p1) return p2;
    if (!p2) return p1;
    
    const [x1, y1] = p1;
    const [x2, y2] = p2;
    
    if (x1 === x2) {
        if (y1 === y2) {
            // Point doubling
            const s = mod((3n * x1 * x1 + P256.a) * modInverse(2n * y1, P256.p), P256.p);
            const x3 = mod(s * s - 2n * x1, P256.p);
            const y3 = mod(s * (x1 - x3) - y1, P256.p);
            return [x3, y3];
        } else {
            // Points are inverses
            return null;
        }
    }
    
    const s = mod((y2 - y1) * modInverse(x2 - x1, P256.p), P256.p);
    const x3 = mod(s * s - x1 - x2, P256.p);
    const y3 = mod(s * (x1 - x3) - y1, P256.p);
    
    return [x3, y3];
}

// Scalar multiplication using double-and-add
function scalarMult(k, point) {
    if (k === 0n) return null;
    if (k === 1n) return point;
    
    let result = null;
    let addend = point;
    
    while (k > 0n) {
        if (k & 1n) {
            result = pointAdd(result, addend);
        }
        addend = pointAdd(addend, addend);
        k >>= 1n;
    }
    
    return result;
}

function loadSampleKeys() {
    document.getElementById('privateKey').value = 'c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5';
    document.getElementById('privateKeyB').value = 'f8f8a2f43c8376ccb0871305060d7b27b0554d2cc72bccf41b2705608452f315';
}

function updatePublicKeysDisplay(privateKeyA, privateKeyB) {
    const publicKeyAX = document.getElementById('publicKeyAX');
    const publicKeyAY = document.getElementById('publicKeyAY');
    const publicKeyBX = document.getElementById('publicKeyBX');
    const publicKeyBY = document.getElementById('publicKeyBY');
    
    // Calculate and display Party A public key
    if (privateKeyA) {
        const publicKeyAPoint = scalarMult(privateKeyA, [P256.Gx, P256.Gy]);
        if (publicKeyAPoint) {
            const [qax, qay] = publicKeyAPoint;
            publicKeyAX.value = qax.toString(16).padStart(64, '0');
            publicKeyAY.value = qay.toString(16).padStart(64, '0');
        }
    }
    
    // Calculate and display Party B public key
    if (privateKeyB) {
        const publicKeyBPoint = scalarMult(privateKeyB, [P256.Gx, P256.Gy]);
        if (publicKeyBPoint) {
            const [qbx, qby] = publicKeyBPoint;
            publicKeyBX.value = qbx.toString(16).padStart(64, '0');
            publicKeyBY.value = qby.toString(16).padStart(64, '0');
        }
    }
}

function calculatePublicKey() {
    const errorDiv = document.getElementById('error');
    const publicKeyAX = document.getElementById('publicKeyAX');
    const publicKeyAY = document.getElementById('publicKeyAY');
    const publicKeyBX = document.getElementById('publicKeyBX');
    const publicKeyBY = document.getElementById('publicKeyBY');
    
    // Clear previous results (but NOT the shared secret)
    errorDiv.innerHTML = '';
    publicKeyAX.value = '';
    publicKeyAY.value = '';
    publicKeyBX.value = '';
    publicKeyBY.value = '';
    // DON'T clear shared secret: document.getElementById('sharedSecret').value = '';
    
    try {
        const privateKeyAHex = document.getElementById('privateKey').value.trim();
        const privateKeyBHex = document.getElementById('privateKeyB').value.trim();
        
        let privateKeyA = null;
        let privateKeyB = null;
        
        // Calculate Party A public key
        if (privateKeyAHex) {
            validatePrivateKey(privateKeyAHex, 'Private Key A');
            privateKeyA = BigInt('0x' + privateKeyAHex);
            const publicKeyAPoint = scalarMult(privateKeyA, [P256.Gx, P256.Gy]);
            
            if (publicKeyAPoint) {
                const [qax, qay] = publicKeyAPoint;
                publicKeyAX.value = qax.toString(16).padStart(64, '0');
                publicKeyAY.value = qay.toString(16).padStart(64, '0');
            }
        }
        
        // Calculate Party B public key
        if (privateKeyBHex) {
            validatePrivateKey(privateKeyBHex, 'Private Key B');
            privateKeyB = BigInt('0x' + privateKeyBHex);
            const publicKeyBPoint = scalarMult(privateKeyB, [P256.Gx, P256.Gy]);
            
            if (publicKeyBPoint) {
                const [qbx, qby] = publicKeyBPoint;
                publicKeyBX.value = qbx.toString(16).padStart(64, '0');
                publicKeyBY.value = qby.toString(16).padStart(64, '0');
            }
        }
        
    } catch (error) {
        errorDiv.innerHTML = '<div class="notice--danger"><strong>Error:</strong> ' + error.message + '</div>';
    }
}

function validatePrivateKey(keyHex, keyName) {
    if (!keyHex) {
        throw new Error(`Please enter ${keyName}`);
    }
    
    if (!/^[0-9a-fA-F]+$/.test(keyHex)) {
        throw new Error(`${keyName} must contain only hexadecimal characters (0-9, a-f, A-F)`);
    }
    
    if (keyHex.length !== 64) {
        throw new Error(`${keyName} must be exactly 64 hexadecimal characters (32 bytes) for P-256`);
    }
    
    const privateKey = BigInt('0x' + keyHex);
    if (privateKey <= 0n || privateKey >= P256.n) {
        throw new Error(`${keyName} must be between 1 and n-1 where n is the curve order`);
    }
}

function performKeyExchange() {
    const errorDiv = document.getElementById('error');
    const sharedSecretField = document.getElementById('sharedSecret');
    
    // Debug: Check if we can find the shared secret field
    console.log('Shared secret field found:', !!sharedSecretField);
    if (!sharedSecretField) {
        console.error('Could not find sharedSecret element!');
        return;
    }
    
    // Clear previous error
    errorDiv.innerHTML = '';
    sharedSecretField.value = '';
    
    try {
        const privateKeyAHex = document.getElementById('privateKey').value.trim();
        const privateKeyBHex = document.getElementById('privateKeyB').value.trim();
        
        // Validate both private keys
        validatePrivateKey(privateKeyAHex, 'Private Key A');
        validatePrivateKey(privateKeyBHex, 'Private Key B');
        
        const privateKeyA = BigInt('0x' + privateKeyAHex);
        const privateKeyB = BigInt('0x' + privateKeyBHex);
        
        console.log('Private Key A:', privateKeyAHex);
        console.log('Private Key B:', privateKeyBHex);
        
        // Calculate public keys first
        const publicKeyA = scalarMult(privateKeyA, [P256.Gx, P256.Gy]);
        const publicKeyB = scalarMult(privateKeyB, [P256.Gx, P256.Gy]);
        
        if (!publicKeyA || !publicKeyB) {
            throw new Error('Failed to calculate public keys');
        }
        
        console.log('Public Key A:', publicKeyA[0].toString(16), publicKeyA[1].toString(16));
        console.log('Public Key B:', publicKeyB[0].toString(16), publicKeyB[1].toString(16));
        
        // Perform ECDH: 
        // Party A calculates: sharedSecret = privateKeyA * publicKeyB
        // Party B calculates: sharedSecret = privateKeyB * publicKeyA
        // Both should get the same result
        const sharedSecretA = scalarMult(privateKeyA, publicKeyB);
        const sharedSecretB = scalarMult(privateKeyB, publicKeyA);
        
        console.log('Shared Secret A:', sharedSecretA ? sharedSecretA[0].toString(16) : 'null');
        console.log('Shared Secret B:', sharedSecretB ? sharedSecretB[0].toString(16) : 'null');
        
        if (!sharedSecretA || !sharedSecretB) {
            throw new Error('Failed to calculate shared secret. Check browser console for details.');
        }
        
        // Verify both parties get the same shared secret
        if (sharedSecretA[0] !== sharedSecretB[0] || sharedSecretA[1] !== sharedSecretB[1]) {
            throw new Error(`Shared secret mismatch! A: ${sharedSecretA[0].toString(16)}, B: ${sharedSecretB[0].toString(16)}`);
        }
        
        console.log('ECDH calculation successful. Both parties have matching shared secrets.');
        console.log('Shared secret point:', sharedSecretA[0].toString(16), sharedSecretA[1].toString(16));
        
        // Display the shared secret (typically only X coordinate is used)
        const sharedSecretHex = sharedSecretA[0].toString(16).padStart(64, '0');
        
        // Set the shared secret first, before updating public keys
        sharedSecretField.value = sharedSecretHex;
        
        // Update public keys display (but don't clear the shared secret)
        updatePublicKeysDisplay(privateKeyA, privateKeyB);
        
        // Additional debug to ensure the field is being set
        console.log('Setting shared secret field to:', sharedSecretHex);
        console.log('Shared secret field element:', sharedSecretField);
        console.log('Field value after setting:', sharedSecretField.value);
        console.log('Field innerHTML after setting:', sharedSecretField.innerHTML);
        
        // Also update public keys if they weren't calculated yet
        calculatePublicKey();
        
        // Add success message
        errorDiv.innerHTML = '<div class="notice--success"><strong>Success!</strong> ECDH key exchange completed. Both parties now share the same secret.</div>';
        
    } catch (error) {
        console.error('ECDH Error:', error);
        errorDiv.innerHTML = '<div class="notice--danger"><strong>Error:</strong> ' + error.message + '</div>';
    }
}

// Allow Enter key to trigger calculation
document.getElementById('privateKey').addEventListener('keypress', function(e) {
    if (e.key === 'Enter') {
        calculatePublicKey();
    }
});

document.getElementById('privateKeyB').addEventListener('keypress', function(e) {
    if (e.key === 'Enter') {
        calculatePublicKey();
    }
});
</script>

## ECDH Key Exchange

The Elliptic Curve Diffie-Hellman (ECDH) key exchange allows two parties to establish a shared secret over an insecure channel:

1. **Party A** generates private key `a` and calculates public key `A = a × G`
2. **Party B** generates private key `b` and calculates public key `B = b × G`
3. Both parties exchange their public keys
4. **Party A** calculates shared secret: `S = a × B`
5. **Party B** calculates shared secret: `S = b × A`
6. Both parties arrive at the same shared secret `S = a × b × G`

### Security Properties

- The shared secret cannot be derived by an eavesdropper who only knows the public keys
- Based on the computational difficulty of the Elliptic Curve Discrete Logarithm Problem (ECDLP)
- The X coordinate of the shared point is typically used as the shared secret for further cryptographic operations

## How It Works

The tool implements elliptic curve point multiplication using the NIST P-256 curve parameters:

- **Prime field**: `p = 2^256 - 2^224 + 2^192 + 2^96 - 1`
- **Curve equation**: `y² = x³ - 3x + b (mod p)`
- **Base point**: Pre-defined generator point G
- **Public key calculation**: `Q = k × G` where k is the private key

### Key Features

- **Input validation**: Ensures private key is valid hex and within curve order
- **Secure arithmetic**: Uses BigInt for precise modular arithmetic
- **Efficient algorithm**: Double-and-add method for scalar multiplication
- **Clear output**: Separate display of X and Y coordinates

### Security Notes

- Private keys must be randomly generated and kept secure
- This tool is for educational/development purposes
- Never enter real private keys in web tools for production use

---

*This tool implements the mathematical foundations of elliptic curve cryptography as specified in NIST standards.*