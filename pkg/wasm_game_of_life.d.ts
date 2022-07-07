/* tslint:disable */
/* eslint-disable */
/**
*/
export function init_panic_hook(): void;
/**
* @param {ElectricField} electric_field
*/
export function main(electric_field: ElectricField): void;
/**
*/
export class ElectricField {
  free(): void;
/**
* @returns {ElectricField}
*/
  static new(): ElectricField;
/**
* @returns {number}
*/
  width(): number;
/**
* @returns {number}
*/
  height(): number;
/**
* @returns {number}
*/
  cqn(): number;
/**
* @returns {number}
*/
  get_pointer(): number;
/**
* @param {number} q
* @param {number} x
* @param {number} y
*/
  install_charge(q: number, x: number, y: number): void;
/**
* @returns {number}
*/
  cx0(): number;
/**
* @returns {number}
*/
  cy0(): number;
/**
* @returns {number}
*/
  cvx0(): number;
/**
* @returns {number}
*/
  cvy0(): number;
/**
* @returns {number}
*/
  cax0(): number;
/**
* @returns {number}
*/
  cay0(): number;
/**
* @returns {number}
*/
  cx1(): number;
/**
* @returns {number}
*/
  cy1(): number;
/**
* @returns {number}
*/
  cvx1(): number;
/**
* @returns {number}
*/
  cvy1(): number;
/**
* @returns {number}
*/
  cax1(): number;
/**
* @returns {number}
*/
  cay1(): number;
}
