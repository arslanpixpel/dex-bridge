import styled from "styled-components";
import theme from "../../../theme/theme";
// import theme from "../../../theme";
import { Family, TextAlignment, ThemeColor, Weight } from "../../../types/components/atoms/text/text";

export const StyledText = styled.div<{
    fontSize: string;
    fontWeight: Weight;
    fontColor: ThemeColor;
    fontLetterSpacing?: string;
    fontFamily: Family;
    align?: TextAlignment;
}>`
    font-family: ${({ fontFamily }) => theme.font.family[fontFamily]};
    font-size: ${({ fontSize }) => fontSize + "px"};
    font-weight: ${({ fontWeight }) => theme.font.weight[fontWeight]};
    color: ${({ fontColor }) => theme.colors[fontColor]};
    letter-spacing: ${({ fontLetterSpacing }) => fontLetterSpacing + "px"};
    text-align: ${({ align }) => align};
`;
