---
layout: splash
title: SHA-256 Hash from Hex string or C Array Input
permalink: /tools/sha256
---
<head>
    <style>
        #inputData, #outputResult {
            background-color: #9BD3FFFF; /* Change this value to your desired color */
            background-blend-mode: multiply
        }
    </style>
</head>

This page hepls to calculate the SHA-256 value from the input hex string or array of uint8 values in C syntax.

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

  // Check if the input contains curly braces (C array syntax)
  var isCArray = inputData.includes('{') && inputData.includes('}');

    if (isCArray) 
    {
        var hexString = convertDecHex(inputData)
    }
    else{

      inputData = inputData.replace(/[^0-9a-fA-F,{}\s]/g, '');
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

function convertDecHex(inStr) {
    let parsedValues = [];
//convert str to arr str
    let aStr =  inStr.replaceAll(/[{},'\[\]]/g, " ").split(/\s+/g);
    aStr.forEach((str) => {
            if (str.length !== 0)
                str.includes('0x') ?
                    (  str.length===3? parsedValues.push(str.replace('0x', '0')):
                        parsedValues.push(str.replace('0x', '')) ):
                    (  (+str).toString(16).length===1? parsedValues.push('0'+(+str).toString(16)):
                        parsedValues.push(((+str).toString(16))))
        }
    )
    return parsedValues.join('');
}

</script>
