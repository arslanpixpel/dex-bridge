import { createContext } from "react";

export type AppContext = {
    isTablet: boolean;
    isMobile: boolean;
};

export const appContext = createContext<AppContext>({ isTablet: false, isMobile: false });
