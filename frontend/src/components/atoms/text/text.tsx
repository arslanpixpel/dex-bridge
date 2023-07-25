import { FC, HTMLAttributes } from "react";
import { TextProps } from "../../../types/components/atoms/text/text";
import { StyledText } from "./text.style";
const Text: FC<TextProps & HTMLAttributes<HTMLDivElement>> = ({
    fontSize = "14",
    fontColor = "TitleText",
    fontWeight = "regular",
    fontFamily = "Roboto",
    fontLetterSpacing = "0",
    children,
    ...props
}) => {
    return (
        <StyledText
            fontSize={fontSize}
            fontWeight={fontWeight}
            fontColor={fontColor}
            fontLetterSpacing={fontLetterSpacing}
            fontFamily={fontFamily}
            {...props}
        >
            {children}
        </StyledText>
    );
};
export default Text;
