import React from "react";
import { StyledMain } from "./PageWrapper.style";

interface Props {
    children?: React.ReactNode;
    className?: string;
}

const PageWrapper = ({ children, className }: Props): JSX.Element => {
    return (
        <StyledMain style={{ opacity: 0 }} className={className}>
            {children}
        </StyledMain>
    );
};

export default PageWrapper;
