'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { motion } from 'motion/react';
import { $api } from '@lib/providers/api';
import { IconSettings, IconLayoutGrid, IconLogout, IconArrowRight } from '@tabler/icons-react';
import { Button } from '@components/ui/button';
import Link from 'next/link';

function AppCard({ name, icon }: { name: string; icon?: string | null }) {
    return (
        <div className="flex flex-col items-center gap-3 rounded-xl border border-border bg-card p-6 hover:bg-accent/50 transition-colors cursor-pointer">
            {icon ? (
                <img src={icon} alt={name} className="size-12 rounded-lg" />
            ) : (
                <div className="size-12 rounded-lg bg-primary/10 flex items-center justify-center">
                    <span className="text-lg font-semibold text-primary">
                        {name.charAt(0).toUpperCase()}
                    </span>
                </div>
            )}
            <span className="text-sm font-medium text-foreground">{name}</span>
        </div>
    );
}

function PortalSkeleton() {
    return (
        <div className="min-h-screen bg-background flex items-center justify-center">
            <div className="size-8 rounded-full border-2 border-primary border-t-transparent animate-spin" />
        </div>
    );
}

export default function HomePage() {
    const router = useRouter();

    const profileQuery = $api.useQuery(
        'get',
        '/api/settings/profile',
        {},
        {
            retry: false,
            refetchOnWindowFocus: false,
        }
    );

    const appsQuery = $api.useQuery(
        'get',
        '/api/applications',
        {},
        {
            retry: false,
            refetchOnWindowFocus: false,
            enabled: !!profileQuery.data,
        }
    );

    useEffect(() => {
        const err = profileQuery.error as unknown as { error?: string } | null;
        if (err?.error === 'Unauthorized') {
            router.replace('/login');
        }
    }, [profileQuery.error, router]);

    const logoutMutation = $api.useMutation('post', '/api/logout', {
        onSuccess: () => router.replace('/login'),
    });

    if (profileQuery.isLoading || (!profileQuery.data && !profileQuery.error)) {
        return <PortalSkeleton />;
    }

    const profile = profileQuery.data;
    if (!profile) return null;

    const apps = appsQuery.data ?? [];

    return (
        <div className="min-h-screen bg-background">
            <div className="mx-auto max-w-3xl px-6 py-14">
                <motion.div
                    initial={{ opacity: 0, y: 8 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ duration: 0.3 }}
                    className="space-y-8"
                >
                    {/* Header */}
                    <div className="flex items-center justify-between">
                        <div>
                            <h1 className="text-2xl font-semibold">
                                Welcome back, {profile.display_name}
                            </h1>
                            <p className="text-sm text-muted-foreground mt-1">{profile.email}</p>
                        </div>
                        <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => logoutMutation.mutate({})}
                            disabled={logoutMutation.isPending}
                            className="text-muted-foreground"
                        >
                            <IconLogout className="size-4" />
                            Sign out
                        </Button>
                    </div>

                    {/* Applications */}
                    <div>
                        <div className="flex items-center gap-2 mb-4">
                            <IconLayoutGrid className="size-5 text-muted-foreground" />
                            <h2 className="text-lg font-medium">Applications</h2>
                        </div>
                        {apps.length > 0 ? (
                            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
                                {apps.map((app) => (
                                    <AppCard key={app.slug} name={app.name} icon={app.icon} />
                                ))}
                            </div>
                        ) : (
                            <div className="rounded-xl border border-dashed border-border p-8 text-center">
                                <IconLayoutGrid className="size-8 text-muted-foreground/50 mx-auto mb-2" />
                                <p className="text-sm text-muted-foreground">
                                    No applications available
                                </p>
                            </div>
                        )}
                    </div>

                    {/* Account Settings Card */}
                    <Link href="/dashboard" className="block">
                        <div className="rounded-xl border border-border bg-card p-5 hover:bg-accent/50 transition-colors flex items-center justify-between group">
                            <div className="flex items-center gap-3">
                                <div className="size-10 rounded-lg bg-primary/10 flex items-center justify-center">
                                    <IconSettings className="size-5 text-primary" />
                                </div>
                                <div>
                                    <h3 className="text-sm font-medium">Account Settings</h3>
                                    <p className="text-xs text-muted-foreground">
                                        Security, sessions, and profile
                                    </p>
                                </div>
                            </div>
                            <IconArrowRight className="size-4 text-muted-foreground group-hover:translate-x-0.5 transition-transform" />
                        </div>
                    </Link>
                </motion.div>
            </div>
        </div>
    );
}
