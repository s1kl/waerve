export function mouse_coordinate(e) {
    const rect = e.target.getBoundingClientRect()
    const viewX = e.clientX - rect.left
    const viewY = e.clientY - rect.top
    const scaleWidth =  this.canvasClientWidth / this.canvasWidth
    const scaleHeight = this.canvasClientHeight / this.canvasHeight
    const canvasX = Math.floor(viewX / scaleWidth)
    const canvasY = Math.floor(viewY / scaleHeight)
    // console.log(canvasX, canvasY)
    this.electricField.install_charge(1.0, canvasX, canvasY, this.canvasWidth, this.canvasHeight)
}
