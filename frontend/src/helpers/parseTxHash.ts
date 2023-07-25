const parseTxHash = (receipt: string) => {
    const length = receipt.length;
    const left = receipt.slice(0, 3);
    const right = receipt.slice(length - 5, length);
    return `${left}...${right}`;
};

export default parseTxHash;
