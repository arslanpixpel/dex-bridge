import styled from "styled-components";

export const LogoWrapper = styled.div<{ logo: "ccd" | "ccp" }>`
    position: absolute;
    ${({ logo }) => logo === "ccd" && `> :first-child{transform:translateY(-1px);}`}
    ${({ logo }) => logo === "ccd" && "bottom: 24px;"}
    ${({ logo }) => logo === "ccp" && "cursor: pointer;"}
    ${({ logo }) => logo === "ccd" && "right: 32px;"}
    ${({ logo }) => logo === "ccp" && "top: 24px;"}
    ${({ logo }) => logo === "ccp" && "left: 32px;"}
    display: flex;
    align-items: center;
    height: ${({ logo }) => (logo === "ccd" ? "18px" : "44px")};

    @media only screen and (max-width: 1050px) {
        position: ${({ logo }) => (logo === "ccd" ? "absolute" : "relative")};

        ${({ logo }) => logo === "ccd" && "bottom: 8px;"}
        ${({ logo }) => logo === "ccd" && "right: 8px;"}
        ${({ logo }) => logo === "ccp" && "top: 0;"}
        ${({ logo }) => logo === "ccp" && "left: 0;"}
        
        height: ${({ logo }) => (logo === "ccd" ? "9px" : "22px")};
    }
`;
