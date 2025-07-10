import { cn } from '@lib/utils';
import { Icon } from '@tabler/icons-react';
import React from 'react';

export type LoginOptionProps = React.ComponentProps<'button'> & {
    title?: string;
    description?: string;
    rightSection?: React.ReactNode;
    icon?: Icon;
    clickable?: boolean;
    selected?: boolean;
};

export function LoginOption({
    title,
    description,
    rightSection,
    className,
    icon: Icon,
    children,
    clickable = false,
    selected = false,
    ...props
}: LoginOptionProps) {
    return (
        <button
            className={cn(
                'p-4 rounded-lg flex justify-between items-center border mt-4 gap-2',
                'transition-all outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]',
                clickable && 'cursor-pointer hover:bg-card/50',
                selected && 'border-transparent outline-2 outline-primary -outline-offset-1',
                className
            )}
            {...props}
        >
            <div className="flex gap-3 items-center">
                {Icon && <Icon className="size-6 text-muted-foreground" />}
                <div className="flex-1 flex flex-col gap-0.5">
                    {title && <div className="font-medium">{title}</div>}
                    {description && (
                        <div className="text-xs text-muted-foreground">{description}</div>
                    )}
                    {children}
                </div>
            </div>
            <div>{rightSection}</div>
        </button>
    );
}
