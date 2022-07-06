export function mouse_coordinate(e) {
    const rect = e.target.getBoundingClientRect()
    const viewX = e.clientX - rect.left
    const viewY = e.clientY - rect.top
    // const scaleWidth = canvasClientWidth / canvasWidth
    // const scaleHeight = canvasClientHeight / canvasHeight
    const canvasX = Math.floor(viewX / this.scaleWidth)
    const canvasY = Math.floor(viewY / this.scaleHeight)
    console.log(canvasX, canvasY)
}
