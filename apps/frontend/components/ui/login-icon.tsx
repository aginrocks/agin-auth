import { cn } from '@lib/utils';

export type LoginIconProps = React.ComponentProps<'div'>;

export function LoginIcon({ children, className, ...props }: LoginIconProps) {
    return (
        <div
            className={cn(
                "size-14 rounded-xl border border-border/70 dark:border-border shadow-md flex justify-center items-center [&_svg:not([class*='size-'])]:size-6 text-muted-foreground",
                className
            )}
        >
            {children}
        </div>
    );
}
