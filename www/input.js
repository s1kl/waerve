export function install(e) {
    const xy = get_mouse_coordinate(e, this.canvasClientWidth,  this.canvasClientHeight, this.canvasWidth, this.canvasHeight)
    this.electricField.install_charge(1.0, xy[0], xy[1], this.canvasWidth, this.canvasHeight)
}

export function remove(e) {
    const xy = get_mouse_coordinate(e, this.canvasClientWidth,  this.canvasClientHeight, this.canvasWidth, this.canvasHeight)
    console.log(xy)
    this.electricField.remove_charge(xy[0], xy[1], this.canvasWidth, this.canvasHeight)
}

function get_mouse_coordinate(e, canvasClientWidth,  canvasClientHeight, canvasWidth, canvasHeight) {
    const rect = e.target.getBoundingClientRect()
    return [Math.floor((e.clientX - rect.left) * canvasWidth / canvasClientWidth), Math.floor((e.clientY - rect.top) * canvasHeight / canvasClientHeight)]
}