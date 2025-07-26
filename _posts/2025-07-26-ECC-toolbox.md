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
    <label for="privateKey"><strong>Private Key (Hex):</strong></label>
    <input type="text" id="privateKey" class="form-control" 
           placeholder="Enter 64-character hexadecimal private key"
           maxlength="64" style="font-family: monospace;">
    <small class="form-text text-muted">Example: c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5</small>
  </div>

  <div class="text-center">
    <button type="button" onclick="loadSampleKey()" class="btn btn--info btn--small">Load Sample Key</button>
    <button type="button" onclick="calculatePublicKey()" class="btn btn--primary">Calculate Public Key</button>
  </div>
</form>

<div id="results" style="margin-top: 2rem;">
  <h3>Public Key (Uncompressed Format)</h3>
  
  <div class="form-group">
    <label for="publicKeyX"><strong>X Coordinate:</strong></label>
    <textarea id="publicKeyX" class="form-control" readonly rows="2" 
              placeholder="X coordinate will appear here..." 
              style="font-family: monospace; font-size: 0.9rem;"></textarea>
  </div>

  <div class="form-group">
    <label for="publicKeyY"><strong>Y Coordinate:</strong></label>
    <textarea id="publicKeyY" class="form-control" readonly rows="2" 
              placeholder="Y coordinate will appear here..." 
              style="font-family: monospace; font-size: 0.9rem;"></textarea>
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

function loadSampleKey() {
    document.getElementById('privateKey').value = 'c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5';
}

function calculatePublicKey() {
    const errorDiv = document.getElementById('error');
    const publicKeyX = document.getElementById('publicKeyX');
    const publicKeyY = document.getElementById('publicKeyY');
    
    // Clear previous results
    errorDiv.innerHTML = '';
    publicKeyX.value = '';
    publicKeyY.value = '';
    
    try {
        const privateKeyHex = document.getElementById('privateKey').value.trim();
        
        // Validate input
        if (!privateKeyHex) {
            throw new Error('Please enter a private key');
        }
        
        if (!/^[0-9a-fA-F]+$/.test(privateKeyHex)) {
            throw new Error('Private key must contain only hexadecimal characters (0-9, a-f, A-F)');
        }
        
        if (privateKeyHex.length !== 64) {
            throw new Error('Private key must be exactly 64 hexadecimal characters (32 bytes) for P-256');
        }
        
        const privateKey = BigInt('0x' + privateKeyHex);
        
        // Validate private key range
        if (privateKey <= 0n || privateKey >= P256.n) {
            throw new Error('Private key must be between 1 and n-1 where n is the curve order');
        }
        
        // Calculate public key: Q = k * G
        const publicKeyPoint = scalarMult(privateKey, [P256.Gx, P256.Gy]);
        
        if (!publicKeyPoint) {
            throw new Error('Failed to calculate public key');
        }
        
        const [qx, qy] = publicKeyPoint;
        
        // Convert to hex strings with proper padding
        const qxHex = qx.toString(16).padStart(64, '0');
        const qyHex = qy.toString(16).padStart(64, '0');
        
        publicKeyX.value = qxHex;
        publicKeyY.value = qyHex;
        
    } catch (error) {
        errorDiv.innerHTML = '<div class="notice--danger"><strong>Error:</strong> ' + error.message + '</div>';
    }
}

// Allow Enter key to trigger calculation
document.getElementById('privateKey').addEventListener('keypress', function(e) {
    if (e.key === 'Enter') {
        calculatePublicKey();
    }
});
</script>

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