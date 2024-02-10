---
layout: splash
title: SHA-256 Hash from Hex string or C Array Input
permalink: /tools/sha256
---
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Password Derivation</title>
</head>
<body>

<h2>Password Derivation</h2>

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
  var seed = document.getElementById("seed").value;
  
  // Password derivation function
  var password = deriveBase64Password(inputString, seed);
  
  document.getElementById("output").innerHTML = "Derived Password (Base64): " + password;
}

function deriveBase64Password(inputString, seed) {
  // Simple concatenation of inputString and seed
  var combinedString = inputString + seed;
  // Encoding the combined string to base64
  var password = btoa(combinedString);
  return password;
}
</script>

</body>
</html>
