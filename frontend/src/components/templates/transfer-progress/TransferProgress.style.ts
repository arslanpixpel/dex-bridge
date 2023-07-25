import Container from "@components/atoms/container/Container";
import theme from "src/theme/theme";
import styled from "styled-components";

export const StyledContainer = styled(Container)`
    gap: 30px;
`;

export const StyledProcessWrapper = styled.div`
    position: relative;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
`;

export const StyledCircleWrapper = styled.div<{ index: number }>`
    display: flex;
    flex-direction: column;
    align-items: ${({ index }) => (index === 1 ? "flex-start" : index === 2 ? "center" : "flex-end")};
    z-index: 1;
    & > :last-child {
        position: absolute;
        top: 125%;
    }
`;

export const StyledCircle = styled.div<{ completed: boolean }>`
    border: 1px solid #000000;
    border-radius: 50%;
    width: 20px;
    height: 20px;
    background-color: ${({ completed }) => (completed ? theme.colors.Black : "#d9d4ce")};

    &::before {
        content: "";
        position: absolute;
        width: 8px;
        height: 12px;
        border-bottom: 3px solid white;
        border-right: 3px solid white;
        opacity: ${({ completed }) => (completed ? 1 : 0)};
        transform: rotate(45deg) translate(69%, -18%);
    }
`;

export const StyledHorizontalLine = styled.hr`
    position: absolute;
    width: 100%;
    border: 1px dashed #707070;
    z-index: 0;
`;

export const StyledButtonContainer = styled.div`
    display: "flex";
    justify-content: "center";
    align-items: "flex-end";
`;

export const TransferAmountWrapper = styled.div`
    width: calc(100% + 80px);
    margin-left: -40px;
    height: 48px;
    background-color: rgba(0, 0, 0, 0.62);
    display: flex;
    align-items: center;
    justify-content: center;
    margin-top: 28px;
`;

export const ModalTitle = styled.div`
    width: 100%;
    height: 80px;
    padding: 28px 40px 0;
    background-color: ${theme.colors.ModalBackground};
    display: flex;
    align-items: center;
    justify-content: center;
`;

export const Content = styled.div`
    width: 100%;
    height: calc(100% - 80px);
    padding: 24px 40px 28px;
    overflow-y: auto;
    background-color: #d9d4ce;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
`;

export const InfoContainer = styled.div<{ processed: boolean }>`
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;

    & > * {
        margin-bottom: 4px !important;
    }

    & > :nth-child(2) {
        ${({ processed }) => processed && "color: #00aa70;"}
    }
`;
