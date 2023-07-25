import { useEffect, useState } from "react";
import { noOp } from "src/helpers/basic";

export const useAsyncMemo = <ReturnType>(
    getResult: () => Promise<ReturnType>,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    handleError: (e: any) => void = noOp,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    deps?: any[]
): ReturnType | undefined => {
    const [result, setResult] = useState<ReturnType>();
    useEffect(() => {
        getResult().then(setResult).catch(handleError);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, deps);
    return result;
};
