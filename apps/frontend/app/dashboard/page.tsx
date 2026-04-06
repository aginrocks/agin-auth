'use client';

import { useCallback, useEffect, useState } from 'react';
import { motion } from 'motion/react';
import { useQueryClient } from '@tanstack/react-query';
import { useRouter } from 'next/navigation';
import { $api } from '@lib/providers/api';
import {
    PasswordRow,
    TotpRow,
    WebAuthnRow,
    PgpRow,
    RecoveryCodesRow,
    ProfileRow,
    DeleteAccountSection,
} from './components';
import { DashboardHeader } from './components/dashboard-header';
import {
    DashboardSkeleton,
    DashboardError,
    DashboardWarning,
} from './components/dashboard-status';

export default function DashboardPage() {
    const queryClient = useQueryClient();
    const router = useRouter();

    const profileQuery = $api.useQuery('get', '/api/settings/profile');
    const factorsQuery = $api.useQuery('get', '/api/settings/factors');

    const [lastProfile, setLastProfile] = useState<typeof profileQuery.data>(profileQuery.data);
    const [lastFactors, setLastFactors] = useState<typeof factorsQuery.data>(factorsQuery.data);

    useEffect(() => {
        if (profileQuery.data) {
            setLastProfile(profileQuery.data);
        }
    }, [profileQuery.data]);

    useEffect(() => {
        if (factorsQuery.data) {
            setLastFactors(factorsQuery.data);
        }
    }, [factorsQuery.data]);

    const profile = profileQuery.data ?? lastProfile;
    const factors = factorsQuery.data ?? lastFactors;

    const logout = $api.useMutation('post', '/api/logout', {
        onSuccess: () => router.push('/login'),
    });

    const refetchFactors = useCallback(() => {
        queryClient.invalidateQueries({ queryKey: ['get', '/api/settings/factors'] });
    }, [queryClient]);

    const handleLogout = useCallback(() => {
        logout.mutate({});
    }, [logout]);

    return (
        <div className="min-h-screen bg-background">
            <div className="mx-auto max-w-2xl px-6 py-14">
                <DashboardHeader
                    profile={profile}
                    onLogout={handleLogout}
                    loggingOut={logout.isPending}
                />

                {profileQuery.isError && !profile && (
                    <div className="mb-6">
                        <DashboardWarning message="Could not load your profile details." />
                    </div>
                )}

                {profileQuery.isError && profile && (
                    <div className="mb-6">
                        <DashboardWarning message="Could not refresh your profile. Showing the last loaded data." />
                    </div>
                )}

                {profile && (
                    <motion.div
                        initial={{ opacity: 0, y: 6 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ duration: 0.25 }}
                        className="mb-6"
                    >
                        <h2 className="text-sm font-medium text-muted-foreground mb-3">Profile</h2>
                        <div className="rounded-2xl border border-border bg-card overflow-hidden">
                            <ProfileRow profile={profile} />
                        </div>
                    </motion.div>
                )}

                {!factors && factorsQuery.isLoading && <DashboardSkeleton />}
                {!factors && factorsQuery.isError && <DashboardError />}

                {factorsQuery.isError && factors && (
                    <div className="mb-6">
                        <DashboardWarning message="Could not refresh security settings. Showing the last loaded data." />
                    </div>
                )}

                {factors && (
                    <motion.div
                        initial={{ opacity: 0, y: 6 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ duration: 0.25 }}
                    >
                        <h2 className="text-sm font-medium text-muted-foreground mb-3">Security</h2>
                        <div className="rounded-2xl border border-border bg-card overflow-hidden">
                            <PasswordRow
                                isSet={factors.password.is_set}
                                onRefetch={refetchFactors}
                            />
                            <TotpRow totp={factors.totp} onRefetch={refetchFactors} />
                            <WebAuthnRow
                                keys={factors.webauthn}
                                onRefetch={refetchFactors}
                            />
                            <PgpRow pgp={factors.pgp} onRefetch={refetchFactors} />
                            <RecoveryCodesRow
                                remaining={factors.recovery_codes.remaining_codes}
                                onRefetch={refetchFactors}
                            />
                        </div>
                    </motion.div>
                )}

                {factors && (
                    <motion.div
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        transition={{ duration: 0.25, delay: 0.1 }}
                        className="mt-8"
                    >
                        <h2 className="text-sm font-medium text-destructive/80 mb-3">
                            Danger Zone
                        </h2>
                        <div className="rounded-2xl border border-destructive/20 bg-card overflow-hidden">
                            <DeleteAccountSection />
                        </div>
                    </motion.div>
                )}
            </div>
        </div>
    );
}
