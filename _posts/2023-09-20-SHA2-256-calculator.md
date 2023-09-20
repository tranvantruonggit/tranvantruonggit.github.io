---
layout: post
title: Calculate SHA-256 Hash in Hex
categories:
  - Tools
---

In this post, we'll create a simple HTML form with JavaScript to calculate the SHA-256 hash of an input string and display the result as a hexadecimal array.

<div>
  <label for="inputString">Enter a string:</label>
  <input type="text" id="inputString" />
</div>

<div>
  <label for="outputResult">Result (SHA-256 Hex):</label>
  <textarea id="outputResult" rows="4" cols="50" readonly></textarea>
</div>

<button onclick="calculateSHA256()">Calculate SHA-256</button>

<script>
  function calculateSHA256() {
    // Get the input value
    var inputElement = document.getElementById("inputString");
    var inputValue = inputElement.value;

    // Calculate the SHA-256 hash
    var hashArray = [];
    var encoder = new TextEncoder();
    var data = encoder.encode(inputValue);
    crypto.subtle.digest("SHA-256", data).then(function(hashBuffer) {
      var hashArray = Array.from(new Uint8Array(hashBuffer));
      var hashHex = hashArray.map(byte => byte.toString(16).padStart(2, '0')).join('');
      
      // Display the result in the output textarea
      var outputElement = document.getElementById("outputResult");
      outputElement.value = hashHex;
    }).catch(function(error) {
      console.error(error);
    });
  }
</script>
