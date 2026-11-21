export class MappingCell {
  id = 0;
  textToString = "";
  textToInteger = 0;
  textToDecimal = "";
  textToDatetime = new Date(0);
  textToBinary: Uint8Array | string = new Uint8Array();
  textToGuid = "";
  intToInteger = 0;
  intToFloat = 0;
  intToDecimal = "";
  intToBoolean = false;
  floatToInteger = 0;
  floatToFloat = 0;
  floatToDecimal = "";
  decimalToInteger = 0;
  decimalToFloat = 0;
  decimalToDecimal = "";
  boolToBoolean = false;
  datetimeToDatetime = new Date(0);
  binaryToBinary: Uint8Array | string = new Uint8Array();
  binaryToGuid = "";
  guidToString = "";
  guidToGuid = "";
}

export const mappingCell = (init: Partial<MappingCell>): MappingCell =>
  Object.assign(new MappingCell(), init);
