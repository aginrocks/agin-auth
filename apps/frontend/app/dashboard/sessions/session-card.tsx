'use client';

import { Button } from '@components/ui/button';
import { IconDevices, IconTrash, IconCircleCheckFilled } from '@tabler/icons-react';
import { parseUserAgent, timeAgo } from '@lib/utils';

type SessionListItem = {
    id: string;
    ip_address: string;
    user_agent: string;
    created_at: string;
    last_active: string;
    current: boolean;
};

interface SessionCardProps {
    session: SessionListItem;
    isLast: boolean;
    onRevoke: (id: string) => void;
    revokeIsPending: boolean;
}

export function SessionCard({ session, isLast, onRevoke, revokeIsPending }: SessionCardProps) {
    const { browser, os } = parseUserAgent(session.user_agent);

    return (
        <div
            className={`px-5 py-4 flex items-center gap-4 ${!isLast ? 'border-b border-border/60' : ''}`}
        >
            <IconDevices className="size-5 text-muted-foreground shrink-0" />
            <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                    <span className="text-sm font-medium">
                        {browser} on {os}
                    </span>
                    {session.current && (
                        <span className="inline-flex items-center gap-1 text-[11px] text-emerald-600 dark:text-emerald-400">
                            <IconCircleCheckFilled className="size-3" />
                            Current
                        </span>
                    )}
                </div>
                <p className="text-xs text-muted-foreground mt-0.5">
                    {session.ip_address}
                    <span className="mx-1.5 text-muted-foreground/40">·</span>
                    Active {timeAgo(session.last_active)}
                </p>
            </div>
            {!session.current && (
                <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => onRevoke(session.id)}
                    disabled={revokeIsPending}
                    className="text-destructive hover:text-destructive hover:bg-destructive/10"
                >
                    <IconTrash className="size-4" />
                    Revoke
                </Button>
            )}
        </div>
    );
}
