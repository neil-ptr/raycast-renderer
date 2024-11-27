const main = () => {
  const canvas = document.getElementById("game-viewport") as HTMLCanvasElement;

  if (!canvas) throw new Error("Canvas didn't load");

  const ctx = canvas.getContext("2d");
  if (!ctx) throw new Error("Failed to get canvas context");

  ctx.fillStyle = "red";

  ctx.fillRect(50, 50, 10, 10);
};

main();
