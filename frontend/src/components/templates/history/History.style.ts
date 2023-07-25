import Container from "@components/atoms/container/Container";
import PageWrapper from "@components/atoms/page-wrapper/PageWrapper";
import Text from "@components/atoms/text/text";
import theme from "src/theme/theme";
import styled from "styled-components";

export const ContentWrapper = styled(PageWrapper)``;

export const HistoryWrapper = styled(Container)`
    overflow: hidden;
`;

export const TableTitle = styled.div`
    width: 100%;
    height: 64px;
    padding: 28px 40px 22px;
    background-color: ${theme.colors.ModalBackground};
    display: flex;
    align-items: center;
`;

export const TableWrapper = styled.div`
    width: 100%;
    height: calc(100% - 100px);
    padding: 24px 20px 28px;
    overflow-y: auto;
    background-color: #d9d4ce;

    overflow-y: auto;
    scrollbar-width: thin; //Firefox
    scrollbar-color: ${theme.colors.TextBrown +
    " " + //space here to not break with prettier
    "#d9d4ce"};

    //Rest of browsers (I think)
    ::-webkit-scrollbar {
        width: 8px;
    }
    ::-webkit-scrollbar-track {
        background: #d9d4ce;
    }
    ::-webkit-scrollbar-thumb {
        background: ${theme.colors.TextBrown};
    }
`;

export const HistoryTable = styled.table`
    width: 100%;
    border-collapse: collapse;
    & > tbody > :last-child {
        border-bottom: none;
    }
    & > thead > tr > :first-child {
        padding-left: 0;
    }
    & > thead > tr > :last-child {
        padding-right: 0;
    }
    & > tbody > tr > :first-child {
        padding-left: 0;
    }
    & > tbody > tr > :last-child {
        padding-right: 0;
    }
    & > thead > tr > th {
        white-space: nowrap;
    }

    @media only screen and (max-width: 540px) {
        & > thead > tr > :first-child {
            padding-left: initial;
        }
        & > tbody > tr > :first-child {
            padding-left: initial;
        }
    }
`;

export const TableHeader = styled.th`
    text-align: left;
`;

export const TableRow = styled.tr`
    cursor: pointer;
    position: relative;
    border-bottom: 1px solid #3232390f;

    &:hover {
        background-color: #cbcbcb;
    }
`;

export const TableData = styled.td`
    white-space: nowrap;
`;

export const LinkWrapper = styled.a`
    margin-top: 32px;
    user-select: none;
    text-decoration: underline;
`;

export const ExplorerLink = styled.a`
    &:hover {
        text-decoration: underline;
    }
`;

export const TabsWrapper = styled.div`
    display: flex;
    background-color: ${theme.colors.ModalBackground};
`;

export const StyledTab = styled.a<{ active: boolean }>`
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    width: 120px;
    height: 36px;
    border-radius: 4px 4px 0 0;
    background-color: ${({ active }) => (active ? theme.colors.ModalBackground : "#d9d4ce")};
`;

export const TransactionStatus = styled(Text)<{ status: string }>`
    background: ${({ status }) => {
        if (status === "Processed") return theme.colors.Green;
        if (status === "Approve") return theme.colors.DarkYellow;
        return theme.colors.DarkGrey;
    }};
    color: theme.colors.Black;
    display: inherit;
    padding: 0 4px;
    border-radius: 3px;
`;
