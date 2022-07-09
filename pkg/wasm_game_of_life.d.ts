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
* @param {number} x
* @param {number} y
*/
  remove_charge(x: number, y: number): void;
/**
* @param {number} x
* @param {number} y
*/
  can_move_charge(x: number, y: number): void;
/**
*/
  cannot_move_charge(): void;
/**
* @returns {boolean}
*/
  test2(): boolean;
}
