'use client';

import { useState, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { IconCheck, IconCopy, IconEye, IconEyeOff } from '@tabler/icons-react';
import { Input } from '@components/ui/input';

export function CopyButton({ text }: { text: string }) {
    const [copied, setCopied] = useState(false);
    return (
        <button
            onClick={() => {
                navigator.clipboard.writeText(text);
                setCopied(true);
                setTimeout(() => setCopied(false), 1500);
            }}
            className="flex items-center gap-1.5 text-xs text-muted-foreground hover:text-foreground transition-colors"
        >
            {copied ? <IconCheck size={13} /> : <IconCopy size={13} />}
            {copied ? 'Copied' : 'Copy'}
        </button>
    );
}

export function ErrorMsg({ msg }: { msg: string }) {
    if (!msg) return null;
    return <p className="text-xs text-destructive mt-2">{msg}</p>;
}

export function ExpandForm({ open, children }: { open: boolean; children: React.ReactNode }) {
    const [overflow, setOverflow] = useState<'hidden' | 'visible'>('hidden');

    useEffect(() => {
        if (open) {
            const t = setTimeout(() => setOverflow('visible'), 400);
            return () => clearTimeout(t);
        } else {
            setOverflow('hidden');
        }
    }, [open]);

    return (
        <AnimatePresence initial={false}>
            {open && (
                <motion.div
                    initial={{ opacity: 0, height: 0 }}
                    animate={{ opacity: 1, height: 'auto' }}
                    exit={{ opacity: 0, height: 0 }}
                    transition={{ type: 'spring', stiffness: 500, damping: 35, mass: 0.8 }}
                    style={{ overflow }}
                >
                    <div className="pt-3 pb-1">{children}</div>
                </motion.div>
            )}
        </AnimatePresence>
    );
}

export function SmoothResize({ children }: { children: React.ReactNode }) {
    const ref = useRef<HTMLDivElement>(null);
    const [height, setHeight] = useState<number | 'auto'>('auto');

    useEffect(() => {
        if (!ref.current) return;
        const ro = new ResizeObserver(([entry]) => setHeight(entry.contentRect.height));
        ro.observe(ref.current);
        return () => ro.disconnect();
    }, []);

    return (
        <motion.div animate={{ height }} transition={{ type: 'spring', stiffness: 500, damping: 35, mass: 0.8 }} style={{ overflow: 'clip' }}>
            <div ref={ref}>{children}</div>
        </motion.div>
    );
}

export function PasswordInput({ value, onChange, placeholder, required, minLength, id }: {
    value: string; onChange: (v: string) => void; placeholder?: string; required?: boolean; minLength?: number; id?: string;
}) {
    const [show, setShow] = useState(false);
    return (
        <div className="relative">
            <Input type={show ? 'text' : 'password'} value={value} onChange={e => onChange(e.target.value)}
                placeholder={placeholder} className="h-9 pr-9 text-sm" required={required} minLength={minLength} id={id} />
            <button type="button" onClick={() => setShow(v => !v)}
                aria-label={show ? 'Hide password' : 'Show password'}
                className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors">
                {show ? <IconEyeOff size={14} /> : <IconEye size={14} />}
            </button>
        </div>
    );
}
