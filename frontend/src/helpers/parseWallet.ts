const parseWallet = (wallet: string) => {
    const len = wallet.length;
    const left = wallet.slice(0, 6);
    const right = wallet.slice(len - 12, len);
    return `${left}...${right}`;
};

export default parseWallet;
