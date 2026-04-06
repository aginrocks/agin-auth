'use client';

import { motion } from 'motion/react';
import { Button } from '@components/ui/button';
import { IconLogout, IconUser } from '@tabler/icons-react';

export function DashboardHeader({ profile, onLogout, loggingOut }: {
    profile?: { display_name: string; email: string; email_confirmed: boolean };
    onLogout: () => void;
    loggingOut: boolean;
}) {
    return (
        <motion.div initial={{ opacity: 0, y: -6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}
            className="mb-8 flex items-start justify-between">
            <div>
                <h1 className="text-xl font-semibold mb-0.5">Account Security</h1>
                {profile ? (
                    <div className="flex flex-wrap items-center gap-2 text-sm text-muted-foreground">
                        <IconUser size={14} />
                        <span>{profile.display_name}</span>
                        <span className="text-muted-foreground/40">·</span>
                        <span>{profile.email}</span>
                        {!profile.email_confirmed && (
                            <span className="inline-flex shrink-0 items-center rounded-full bg-destructive/10 px-1.5 py-0.5 text-[10px] font-medium leading-none text-destructive">
                                Unverified
                            </span>
                        )}
                    </div>
                ) : (
                    <p className="text-sm text-muted-foreground">Keep your account info up to date to ensure you always have access.</p>
                )}
            </div>
            <Button variant="ghost" size="sm" onClick={onLogout} disabled={loggingOut} className="text-muted-foreground hover:text-foreground">
                <IconLogout size={16} />
                {loggingOut ? 'Logging out…' : 'Log out'}
            </Button>
        </motion.div>
    );
}
