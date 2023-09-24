---
layout: splash
title: Calculate SHA-256 Hash from Hex or C Array Input
---

In this post, we'll create a simple HTML form with JavaScript to calculate the SHA-256 hash of input in pure hexadecimal array or C array syntax and display the result as a hexadecimal string.

<div>
  <label for="inputData">Enter a hexadecimal array or C array:</label>
  <input type="text" id="inputData" />
</div>

<div>
  <label for="outputResult">Result (SHA-256 Hex):</label>
  <textarea id="outputResult" rows="4" cols="50" readonly></textarea>
</div>

<button onclick="calculateSHA256()">Calculate SHA-256</button>

<script>
function calculateSHA256() {
  // Get the input value
  var inputElement = document.getElementById("inputData");
  var inputData = inputElement.value;

  // Remove unnecessary characters and spaces
  inputData = inputData.replace(/[^0-9a-fA-F,{}\s]/g, '');

  // Check if the input contains curly braces (C array syntax)
  var isCArray = inputData.includes('{') && inputData.includes('}');

    if (isCArray) 
    {
        var hexString = convertCArrayToHexString(inputData)
    }
    else{

      // Convert the cleaned-up input to a hexadecimal string
      var hexArray = inputData.split(/[\s,{}]+/).filter(Boolean);
      var hexString = hexArray.join('');
    }
  // Convert the hexadecimal string to a Uint8Array
  var hexBytes = new Uint8Array(hexString.length / 2);
  for (var i = 0; i < hexString.length; i += 2) {
    hexBytes[i / 2] = parseInt(hexString.substr(i, 2), 16);
  }

  // Calculate the SHA-256 hash
  crypto.subtle.digest("SHA-256", hexBytes).then(function(hashBuffer) {
    var hashArray = Array.from(new Uint8Array(hashBuffer));
    var hashHex = hashArray.map(byte => byte.toString(16).padStart(2, '0')).join('');

    // Display the result in the output textarea
    var outputElement = document.getElementById("outputResult");
    outputElement.value = hashHex;
  }).catch(function(error) {
    console.error(error);
  });
}

function convertCArrayToHexString(cArrayString) {
  // Remove curly braces and split the string into individual values
  const values = cArrayString
    .replace(/[{}]/g, '') // Remove curly braces
    .split(',')
    .map(valStr => valStr.trim()); // Trim whitespace

  // Convert and format each value as a two-digit hexadecimal string
  const hexArray = values.map(valStr => {
    let val;
    if (valStr.startsWith("0x")) {
      // Handle hexadecimal values with "0x" prefix
      val = parseInt(valStr, 16);
    } else {
      // Handle decimal values
      val = parseInt(valStr, 10);
    }

    if (isNaN(val) || val < 0 || val > 255) {
      throw new Error('Invalid value in the C array string');
    }

    return val.toString(16).padStart(2, '0'); // Convert to hex and pad
  });

  // Join the hex values to form the hex string
  const hexString = hexArray.join('').toUpperCase(); // Optionally convert to uppercase
  return hexString;
}

</script>
