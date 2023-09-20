---
layout: post
title: Square Input with JavaScript
---

In this post, we'll create a simple HTML form with JavaScript to calculate the square of an input value.

<div>
  <label for="inputNumber">Enter a number:</label>
  <input type="text" id="inputNumber" />
</div>

<div>
  <label for="outputResult">Result (square):</label>
  <input type="text" id="outputResult" disabled />
</div>

<button onclick="calculateSquare()">Calculate Square</button>

<script>
  function calculateSquare() {
    // Get the input value
    var inputElement = document.getElementById("inputNumber");
    var inputValue = parseFloat(inputElement.value);

    // Calculate the square
    var squareValue = inputValue * inputValue;

    // Display the result in the output text box
    var outputElement = document.getElementById("outputResult");
    outputElement.value = squareValue;
  }
</script>
