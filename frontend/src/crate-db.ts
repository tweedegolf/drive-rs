export type Category =
  | "Analog"
  | "Analog::ADC"
  | "Analog::DAC"
  | "Sensor"
  | "Sensor::PowerMeter";
export type ShortDependency = string;
export type SpiDeviceType = "SpiBus" | "SpiDevice";
export type Manufacturer = "AnalogDevices" | "ST" | "TI" | "NXP" | "Unknown";
export type Package = string;

export interface FullCrateDb {
  crates: FullCrate[];
  indexes: Indexes;
}

export interface FullCrate {
  description: string;
  categories?: Category[];
  crate_size?: number | null;
  created_at: string;
  datasheets?: string[];
  dependencies: ShortDependency[];
  dev_boards?: DevBoards;
  documentation?: string | null;
  downloads: number;
  homepage?: string | null;
  interfaces?: Interfaces;
  kicad_symbol?: string[];
  license: string;
  manufacturer: Manufacturer;
  name: string;
  names: string[];
  packages?: Package[];
  part_numbers?: string[];
  repository?: string | null;
  resources?: Resource[];
  rust_version?: string | null;
  this_version_downloads: number;
  updated_at: string;
  version: string;
}

export interface DevBoards {
  adafruit?: number | null;
  mikroe?: number | null;
  other?: GenericDevBoard[];
  sparkfun?: number | null;
}

export interface GenericDevBoard {
  link: string;
  name: string;
}

export interface Interfaces {
  i2c?: I2C | null;
  spi?: Spi | null;
}

export interface I2C {
  addrs: number[];
  interrupt: boolean;
}

export interface Spi {
  bus_type: SpiDeviceType;
  interrupt: boolean;
}

export interface Resource {
  title: string;
  link: string;
}

export interface Indexes {
  category: IndexFor_Category;
  dependencies: IndexForString;
  has_dev_board: number[];
  has_kicad: number[];
  interfaces: IndexFor_Interface;
  license: IndexForString;
  package: IndexFor_PackageType;
  rust_version: IndexForString;

  [k: string]: unknown;
}

export interface IndexFor_Category {
  [k: string]: number[];
}

export interface IndexForString {
  [k: string]: number[];
}

export interface IndexFor_Interface {
  [k: string]: number[];
}

export interface IndexFor_PackageType {
  [k: string]: number[];
}
