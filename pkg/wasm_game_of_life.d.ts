/* tslint:disable */
/* eslint-disable */
/**
*/
export function init_panic_hook(): void;
/**
*/
export function init(): void;
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
  get_pointer(): number;
}
