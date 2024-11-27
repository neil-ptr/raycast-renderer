"use strict";
var main = function () {
    var canvas = document.getElementById("game-viewport");
    if (!canvas)
        throw new Error("Canvas didn't load");
    var ctx = canvas.getContext("2d");
    if (!ctx)
        throw new Error("Failed to get canvas context");
    ctx.fillStyle = "red";
    ctx.fillRect(50, 50, 10, 10);
};
main();
