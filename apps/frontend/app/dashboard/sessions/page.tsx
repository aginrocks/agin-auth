'use client';

import { useCallback, useEffect, useState } from 'react';
import { motion } from 'motion/react';
import { $api } from '@lib/providers/api';
import { useQueryClient } from '@tanstack/react-query';
import { useRouter } from 'next/navigation';
import { Button } from '@components/ui/button';
import { IconDevices, IconTrash, IconCircleCheckFilled } from '@tabler/icons-react';
import { DashboardWarning } from '../components/dashboard-status';

type SessionListItem = {
    id: string;
    ip_address: string;
    user_agent: string;
    created_at: string;
    last_active: string;
    current: boolean;
};

function parseUserAgent(ua: string) {
    if (ua === 'unknown') return { browser: 'Unknown', os: 'Unknown' };

    let browser = 'Unknown';
    let os = 'Unknown';

    if (ua.includes('Firefox/')) browser = 'Firefox';
    else if (ua.includes('Edg/')) browser = 'Edge';
    else if (ua.includes('Chrome/')) browser = 'Chrome';
    else if (ua.includes('Safari/')) browser = 'Safari';

    if (ua.includes('iPhone') || ua.includes('iPad')) os = 'iOS';
    else if (ua.includes('Windows')) os = 'Windows';
    else if (ua.includes('Mac OS')) os = 'macOS';
    else if (ua.includes('Android')) os = 'Android';
    else if (ua.includes('Linux')) os = 'Linux';

    return { browser, os };
}

function timeAgo(dateStr: string) {
    const date = new Date(dateStr);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMin = Math.floor(diffMs / 60000);
    const diffHr = Math.floor(diffMin / 60);
    const diffDay = Math.floor(diffHr / 24);

    if (diffMin < 1) return 'just now';
    if (diffMin < 60) return `${diffMin}m ago`;
    if (diffHr < 24) return `${diffHr}h ago`;
    if (diffDay < 30) return `${diffDay}d ago`;
    return date.toLocaleDateString();
}

export default function SessionsPage() {
    const queryClient = useQueryClient();
    const router = useRouter();
    const sessionsQueryKey = ['get', '/api/settings/sessions'] as const;
    const sessionsQuery = $api.useQuery('get', '/api/settings/sessions', {}, {
        retry: false,
        refetchOnWindowFocus: false,
        staleTime: 60_000,
    });
    const [lastSessions, setLastSessions] = useState<SessionListItem[] | undefined>(sessionsQuery.data?.sessions);

    useEffect(() => {
        const err = sessionsQuery.error as unknown as { error?: string } | null;
        if (err?.error === 'Unauthorized') {
            router.push('/login');
        }
    }, [sessionsQuery.error, router]);

    useEffect(() => {
        if (sessionsQuery.data?.sessions) {
            setLastSessions(sessionsQuery.data.sessions);
        }
    }, [sessionsQuery.data]);

    const revokeSession = $api.useMutation('delete', '/api/settings/sessions/{session_id}', {
        onSuccess: (_, variables) => {
            queryClient.setQueriesData(
                { queryKey: sessionsQueryKey },
                (
                    current:
                        | {
                            sessions: SessionListItem[];
                        }
                        | undefined
                ) =>
                    current
                        ? {
                            ...current,
                            sessions: current.sessions.filter(
                                (session) => session.id !== variables.params.path.session_id
                            ),
                        }
                        : current
            );
            setLastSessions((current) =>
                current?.filter(
                    (session) => session.id !== variables.params.path.session_id
                )
            );
            queryClient.invalidateQueries({ queryKey: sessionsQueryKey, refetchType: 'none' });
        },
    });

    const sessions = sessionsQuery.data?.sessions ?? lastSessions;

    const handleRevoke = useCallback(
        (sessionId: string) => {
            revokeSession.mutate({ params: { path: { session_id: sessionId } } });
        },
        [revokeSession]
    );

    return (
        <>
            <motion.div
                initial={{ opacity: 0, y: -6 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ duration: 0.25 }}
                className="mb-8"
            >
                <h1 className="text-xl font-semibold mb-0.5">Sessions</h1>
                <p className="text-sm text-muted-foreground">
                    Manage your active sessions. Revoke any session you don't recognize.
                </p>
            </motion.div>

            {sessionsQuery.isLoading && (
                <div className="rounded-2xl border border-border bg-card">
                    {[...Array(3)].map((_, i) => (
                        <div
                            key={i}
                            className={`px-5 py-4 flex items-center gap-4 ${i < 2 ? 'border-b border-border/60' : ''}`}
                        >
                            <div className="size-5 rounded bg-muted animate-pulse shrink-0" />
                            <div className="flex-1 space-y-2">
                                <div className="h-3.5 w-32 rounded bg-muted animate-pulse" />
                                <div className="h-3 w-48 rounded bg-muted animate-pulse" />
                            </div>
                        </div>
                    ))}
                </div>
            )}

            {sessionsQuery.isError && !sessions && (
                <div className="rounded-2xl border border-border bg-card px-5 py-4">
                    <p className="text-sm text-destructive">
                        Failed to load sessions.
                    </p>
                </div>
            )}

            {sessionsQuery.isError && sessions && (
                <div className="mb-6">
                    <DashboardWarning message="Could not refresh the sessions list. Showing the last loaded data." />
                </div>
            )}

            {sessions && sessions.length === 0 && (
                <div className="rounded-2xl border border-border bg-card px-5 py-8 text-center">
                    <p className="text-sm text-muted-foreground">No active sessions found.</p>
                </div>
            )}

            {sessions && sessions.length > 0 && (
                <motion.div
                    initial={{ opacity: 0, y: 6 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ duration: 0.25 }}
                >
                    <div className="rounded-2xl border border-border bg-card overflow-hidden">
                        {sessions.map((session, i) => {
                            const { browser, os } = parseUserAgent(session.user_agent);
                            return (
                                <div
                                    key={session.id}
                                    className={`px-5 py-4 flex items-center gap-4 ${
                                        i < sessions.length - 1 ? 'border-b border-border/60' : ''
                                    }`}
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
                                            onClick={() => handleRevoke(session.id)}
                                            disabled={revokeSession.isPending}
                                            className="text-destructive hover:text-destructive hover:bg-destructive/10"
                                        >
                                            <IconTrash className="size-4" />
                                            Revoke
                                        </Button>
                                    )}
                                </div>
                            );
                        })}
                    </div>
                </motion.div>
            )}
        </>
    );
}
