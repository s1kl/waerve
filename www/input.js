export function get_mouse_coordinate(e, canvasClientWidth,  canvasClientHeight, canvasWidth, canvasHeight) {
    const rect = e.target.getBoundingClientRect()
    return [Math.floor((e.clientX - rect.left) * canvasWidth / canvasClientWidth), Math.floor((e.clientY - rect.top) * canvasHeight / canvasClientHeight)]
}