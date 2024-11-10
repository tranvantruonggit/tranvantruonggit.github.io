---
layout: splash
title: Toyz password generator
permalink: /tools/passgen
---
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Password Derivation</title>
</head>
<body>

<h2>Password Generator based on AES-MP - Experimental, NOT SECURED!!!</h2>

<form id="passwordForm">
  <label for="inputString">Alphanumeric String:</label><br>
  <input type="text" id="inputString" name="inputString" required><br>
  <label for="seed">Seed:</label><br>
  <input type="text" id="seed" name="seed" required><br><br>
  <button type="button" onclick="derivePassword()">Generate Password</button>
</form>

<div id="output"></div>

<script>
function derivePassword() {
  var inputString = document.getElementById("inputString").value;
  
  var padString = padDataWithPKCS7(inputString);
  var seed = document.getElementById("seed").value;
  
  var prePass = calculateAESBlockCipher(seed, padString);
  handleEncryption();
  // Password derivation function
  var password = deriveBase64Password(prePass);
  
  document.getElementById("output").innerHTML = "Derived Password (Base64): " + password;
}

async function calculateAESBlockCipher(key, data) {
  try {
    // Import the AES key
    const importedKey = await window.crypto.subtle.importKey(
      "raw", 
      key, 
      { name: "AES-ECB" }, 
      false, 
      ["encrypt", "decrypt"]
    );

    // Encrypt the data using AES-ECB
    const encryptedData = await window.crypto.subtle.encrypt(
      { name: "AES-ECB" }, 
      importedKey, 
      data
    );

    // Return the encrypted data
    return encryptedData;
  } catch (error) {
    console.error('AES encryption failed:', error);
    return null;
  }
}

function padDataWithPKCS7(data) {
  const blockSize = 16; // AES block size is 16 bytes
  const paddingLength = blockSize - (data.length % blockSize);
  const paddingValue = paddingLength;
  const paddedData = new Uint8Array([...data, ...Array(paddingLength).fill(paddingValue)]);
  return paddedData;
}

async function handleEncryption() {
  const key = new Uint8Array(16); // Example AES key
  const data = new Uint8Array([/* Your data here */]); // Example data
  
  try {
    const encryptedData = await calculateAESBlockCipher(key, data);
    document.getElementById("output").innerText = "Encrypted Data: " + uint8ArrayToHexString(new Uint8Array(encryptedData));
  } catch (error) {
    console.error('Encryption failed:', error);
    document.getElementById("output").innerText = "Encryption failed";
  }
}

function uint8ArrayToHexString(uint8Array) {
  return Array.from(uint8Array).map(byte => byte.toString(16).padStart(2, '0')).join('');
}


function deriveBase64Password(inputString) {
  // Encoding the combined string to base64
  var password = btoa(inputString);
  return password;
}
</script>

</body>
</html>
