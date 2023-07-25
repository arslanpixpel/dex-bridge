export type ThemeColor =
    | "DarkGrey"
    | "White"
    | "TitleText"
    | "ModalBackground"
    | "Black"
    | "Brown"
    | "TextBrown"
    | "DarkYellow"
    | "Balance"
    | "Red"
    | "Green"
    | "Yellow";

export type Family = "Roboto" | "Helvetica";
export type Weight = "regular" | "light" | "bold";
export type TextAlignment = "left" | "center" | "right";

export type TextProps = {
    fontSize?: string;
    fontColor?: ThemeColor;
    fontWeight?: Weight;
    fontFamily?: Family;
    fontLetterSpacing?: string;
    align?: TextAlignment;
};
