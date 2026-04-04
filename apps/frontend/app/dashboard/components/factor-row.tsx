'use client';

import { motion } from 'motion/react';
import { IconChevronRight } from '@tabler/icons-react';

export function StatusTag({ label, enabled }: { label: string; enabled: boolean }) {
    return (
        <span className={`inline-flex items-center rounded-full px-2 py-0.5 text-[10px] font-medium ${
            enabled
                ? 'bg-foreground/10 text-foreground'
                : 'bg-muted text-muted-foreground'
        }`}>
            {label}
        </span>
    );
}

export function FactorRow({
    icon, name, description, tag, onToggle, children, last, open,
}: {
    icon: React.ReactNode; name: string; description: string; tag: { label: string; enabled: boolean }; onToggle?: () => void; children: React.ReactNode; last?: boolean; open?: boolean;
}) {
    return (
        <div className={`${!last ? 'border-b border-border/60' : ''}`}>
            <button type="button" onClick={onToggle}
                className="w-full px-5 py-4 flex items-center gap-4 text-left hover:bg-muted/50 transition-colors">
                <span className="text-muted-foreground shrink-0 [&_svg]:size-5">{icon}</span>
                <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-0.5">
                        <h3 className="text-sm font-medium leading-tight">{name}</h3>
                        <StatusTag label={tag.label} enabled={tag.enabled} />
                    </div>
                    <p className="text-xs text-muted-foreground">{description}</p>
                </div>
                <motion.span animate={{ rotate: open ? 90 : 0 }} transition={{ type: 'spring', stiffness: 500, damping: 35 }} className="shrink-0">
                    <IconChevronRight size={16} className="text-muted-foreground/40" />
                </motion.span>
            </button>
            {children}
        </div>
    );
}
