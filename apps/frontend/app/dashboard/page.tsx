'use client';

import { useCallback } from 'react';
import { motion } from 'motion/react';
import { useQueryClient } from '@tanstack/react-query';
import { useRouter } from 'next/navigation';
import { $api } from '@lib/providers/api';
import { PasswordRow, TotpRow, WebAuthnRow, PgpRow, RecoveryCodesRow, ProfileRow, DeleteAccountSection } from './components';
import { DashboardHeader } from './components/dashboard-header';
import { DashboardSkeleton, DashboardError } from './components/dashboard-status';

export default function DashboardPage() {
    const queryClient = useQueryClient();
    const router = useRouter();

    const { data: profile } = $api.useQuery('get', '/api/settings/profile');
    const { data, isLoading, isError } = $api.useQuery('get', '/api/settings/factors');

    const logout = $api.useMutation('post', '/api/logout', {
        onSuccess: () => router.push('/login'),
    });

    const refetch = () =>
        queryClient.invalidateQueries({ queryKey: ['get', '/api/settings/factors'] });

    const handleLogout = useCallback(() => {
        logout.mutate({});
    }, [logout]);

    return (
        <div className="min-h-screen bg-background">
            <div className="mx-auto max-w-2xl px-6 py-14">
                <DashboardHeader profile={profile} onLogout={handleLogout} loggingOut={logout.isPending} />

                {profile && (
                    <motion.div initial={{ opacity: 0, y: 6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}
                        className="mb-6">
                        <h2 className="text-sm font-medium text-muted-foreground mb-3">Profile</h2>
                        <div className="rounded-2xl border border-border bg-card overflow-hidden">
                            <ProfileRow profile={profile} />
                        </div>
                    </motion.div>
                )}

                {isLoading && <DashboardSkeleton />}
                {isError && <DashboardError />}

                {data && (
                    <motion.div initial={{ opacity: 0, y: 6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}>
                        <h2 className="text-sm font-medium text-muted-foreground mb-3">Security</h2>
                            <div className="rounded-2xl border border-border bg-card overflow-hidden">
                                <PasswordRow isSet={data.password.is_set} onRefetch={refetch} />
                                <TotpRow totp={data.totp} onRefetch={refetch} />
                                <WebAuthnRow keys={data.webauthn} onRefetch={refetch} />
                                <PgpRow pgp={data.pgp} onRefetch={refetch} />
                                <RecoveryCodesRow remaining={data.recovery_codes.remaining_codes} onRefetch={refetch} />
                            </div>
                    </motion.div>
                )}

                {data && (
                    <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ duration: 0.25, delay: 0.1 }}
                        className="mt-8">
                        <h2 className="text-sm font-medium text-destructive/80 mb-3">Danger Zone</h2>
                        <div className="rounded-2xl border border-destructive/20 bg-card overflow-hidden">
                            <DeleteAccountSection />
                        </div>
                    </motion.div>
                )}
            </div>
        </div>
    );
}
