import type { Component } from "vue";

export type ButtonVariant = "primary" | "outline" | "secondary";
export type ButtonSize = "sm" | "md" | "lg";
export type ButtonType = "button" | "submit" | "reset";

export interface ButtonProps {
   text?: string;
   type?: ButtonType;
   variant?: ButtonVariant;
   size?: ButtonSize;
   disabled?: boolean;
   fullWidth?: boolean;
   prependIcon?: Component;
   appendIcon?: Component;
   className?: string;
}
