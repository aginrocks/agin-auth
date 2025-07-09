import { cn } from '@lib/utils';
import React from 'react';

export function LinkComponent({ className, ...props }: React.ComponentProps<'span'>) {
    return (
        <span
            className={cn(
                'text-foreground cursor-pointer underline hover:text-foreground/90 font-medium',
                className
            )}
            {...props}
        />
    );
}
