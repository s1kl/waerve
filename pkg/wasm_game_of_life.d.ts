/* tslint:disable */
/* eslint-disable */
/**
*/
export function init_panic_hook(): void;
/**
* @param {ElectricField} electric_field
* @param {Charge} charge
*/
export function main(electric_field: ElectricField, charge: Charge): void;
/**
*/
export class Charge {
  free(): void;
/**
* @returns {Charge}
*/
  static new(): Charge;
/**
*/
  q: number;
/**
*/
  x: number;
/**
*/
  y: number;
}
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
  get_pointer(): number;
}
