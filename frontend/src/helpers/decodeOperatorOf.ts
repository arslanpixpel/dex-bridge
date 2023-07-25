const decodeOperatorOf = function (result: string) {
    const buffer = Buffer.from(result, "hex");

    const length = buffer.readUInt16LE(0);

    if (length !== 1) {
        throw new Error(`Invalid length: ${length}. Length should be 1.`);
    }

    const byte = buffer.readUInt8(2);

    if (byte === 0) {
        return false;
    } else if (byte === 1) {
        return true;
    } else {
        throw new Error(`Invalid byte: ${byte}. Byte should be 1 or 0.`);
    }
};

export default decodeOperatorOf;
