'use client';

import { useCallback, useEffect, useState } from 'react';
import { motion } from 'motion/react';
import { $api } from '@lib/providers/api';
import { useQueryClient } from '@tanstack/react-query';
import { useRouter } from 'next/navigation';
import { DashboardWarning, SessionsSkeleton } from '../components/dashboard-status';
import { SessionCard } from './session-card';

type SessionListItem = {
    id: string;
    ip_address: string;
    user_agent: string;
    created_at: string;
    last_active: string;
    current: boolean;
};

export default function SessionsPage() {
    const queryClient = useQueryClient();
    const router = useRouter();
    const sessionsQueryKey = ['get', '/api/settings/sessions'] as const;
    const sessionsQuery = $api.useQuery(
        'get',
        '/api/settings/sessions',
        {},
        {
            retry: false,
            refetchOnWindowFocus: false,
            staleTime: 60_000,
        }
    );
    const [lastSessions, setLastSessions] = useState<SessionListItem[] | undefined>(
        sessionsQuery.data?.sessions
    );

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
                current?.filter((session) => session.id !== variables.params.path.session_id)
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

            {sessionsQuery.isLoading && <SessionsSkeleton />}

            {sessionsQuery.isError && !sessions && (
                <div className="rounded-2xl border border-border bg-card px-5 py-4">
                    <p className="text-sm text-destructive">Failed to load sessions.</p>
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
                        {sessions.map((session, i) => (
                            <SessionCard
                                key={session.id}
                                session={session}
                                isLast={i === sessions.length - 1}
                                onRevoke={handleRevoke}
                                revokeIsPending={revokeSession.isPending}
                            />
                        ))}
                    </div>
                </motion.div>
            )}
        </>
    );
}
