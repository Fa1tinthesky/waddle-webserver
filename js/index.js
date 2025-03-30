let button = document.getElementById("file");

console.log(button);
button.addEventListener('click', function (e) {
    e.preventDefault();
    console.log(button);
});
