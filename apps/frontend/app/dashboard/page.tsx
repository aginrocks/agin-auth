'use client';

import { useState, useEffect, useCallback } from 'react';
import { motion } from 'motion/react';
import { useQueryClient } from '@tanstack/react-query';
import { useRouter } from 'next/navigation';
import { $api } from '@lib/providers/api';
import { Button } from '@components/ui/button';
import { Input } from '@components/ui/input';
import { IconLogout, IconUser, IconTrash, IconCheck, IconX } from '@tabler/icons-react';
import { PasswordRow, TotpRow, WebAuthnRow, PgpRow, RecoveryCodesRow } from './components';
import { PasswordInput, ErrorMsg, ExpandForm } from './components/helpers';
import { FactorRow } from './components/factor-row';

interface Profile {
    preferred_username: string;
    display_name: string;
    email: string;
    email_confirmed: boolean;
    first_name: string;
    last_name: string;
}

export default function DashboardPage() {
    const queryClient = useQueryClient();
    const router = useRouter();
    const { data, isLoading, isError } = $api.useQuery('get', '/api/settings/factors');

    const [profile, setProfile] = useState<Profile | null>(null);
    const [loggingOut, setLoggingOut] = useState(false);
    const [showDelete, setShowDelete] = useState(false);
    const [deletePassword, setDeletePassword] = useState('');
    const [deleteError, setDeleteError] = useState('');
    const [deleting, setDeleting] = useState(false);

    const [editingProfile, setEditingProfile] = useState(false);
    const [profileForm, setProfileForm] = useState({ display_name: '', first_name: '', last_name: '' });
    const [profileSaving, setProfileSaving] = useState(false);
    const [profileError, setProfileError] = useState('');

    const fetchProfile = useCallback(() => {
        fetch('/api/settings/profile').then(r => r.ok ? r.json() : null).then((p: Profile | null) => {
            setProfile(p);
            if (p) setProfileForm({ display_name: p.display_name, first_name: p.first_name, last_name: p.last_name });
        });
    }, []);

    useEffect(() => { fetchProfile(); }, [fetchProfile]);

    const refetch = () =>
        queryClient.invalidateQueries({ queryKey: ['get', '/api/settings/factors'] });

    const handleLogout = useCallback(async () => {
        setLoggingOut(true);
        await fetch('/api/logout', { method: 'POST' });
        router.push('/login');
    }, [router]);

    const handleSaveProfile = useCallback(async (e: React.FormEvent) => {
        e.preventDefault();
        setProfileError('');
        setProfileSaving(true);
        try {
            const res = await fetch('/api/settings/profile', {
                method: 'PATCH',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(profileForm),
            });
            if (res.ok) {
                await fetchProfile();
                setEditingProfile(false);
            } else {
                const d = await res.json().catch(() => null);
                setProfileError(d?.error || 'Failed to save.');
            }
        } catch {
            setProfileError('Failed to save.');
        } finally {
            setProfileSaving(false);
        }
    }, [profileForm, fetchProfile]);

    const handleDeleteAccount = useCallback(async (e: React.FormEvent) => {
        e.preventDefault();
        setDeleteError('');
        setDeleting(true);
        try {
            const res = await fetch('/api/settings/account', {
                method: 'DELETE',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ password: deletePassword }),
            });
            if (res.ok || res.status === 204) {
                router.push('/login');
            } else {
                const data = await res.json().catch(() => null);
                setDeleteError(data?.error || 'Failed to delete account.');
            }
        } catch {
            setDeleteError('Failed to delete account.');
        } finally {
            setDeleting(false);
        }
    }, [deletePassword, router]);

    return (
        <div className="min-h-screen bg-background">
            <div className="mx-auto max-w-2xl px-6 py-14">

                {/* Header */}
                <motion.div initial={{ opacity: 0, y: -6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}
                    className="mb-8 flex items-start justify-between">
                    <div>
                        <h1 className="text-xl font-semibold mb-0.5">Account Security</h1>
                        {profile ? (
                            <div className="flex items-center gap-2 text-sm text-muted-foreground">
                                <IconUser size={14} />
                                <span>{profile.display_name}</span>
                                <span className="text-muted-foreground/40">·</span>
                                <span>{profile.email}</span>
                                {!profile.email_confirmed && (
                                    <span className="text-[10px] bg-destructive/10 text-destructive rounded-full px-1.5 py-0.5">Unverified</span>
                                )}
                            </div>
                        ) : (
                            <p className="text-sm text-muted-foreground">Keep your account info up to date to ensure you always have access.</p>
                        )}
                    </div>
                    <Button variant="ghost" size="sm" onClick={handleLogout} disabled={loggingOut} className="text-muted-foreground hover:text-foreground">
                        <IconLogout size={16} />
                        {loggingOut ? 'Logging out…' : 'Log out'}
                    </Button>
                </motion.div>

                {/* Profile Card */}
                {profile && (
                    <motion.div initial={{ opacity: 0, y: 6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}
                        className="mb-6">
                        <h2 className="text-sm font-medium text-muted-foreground mb-3">Profile</h2>
                        <div className="rounded-2xl border border-border bg-card">
                            <FactorRow
                                icon={<IconUser />}
                                name={profile.display_name}
                                description={[profile.first_name, profile.last_name].filter(Boolean).join(' ') || `@${profile.preferred_username}`}
                                tag={{ label: `@${profile.preferred_username}`, enabled: true }}
                                onToggle={() => {
                                    setEditingProfile(v => !v);
                                    setProfileError('');
                                    setProfileForm({ display_name: profile.display_name, first_name: profile.first_name, last_name: profile.last_name });
                                }}
                                open={editingProfile}
                                last
                            >
                                <ExpandForm open={editingProfile}>
                                    <form onSubmit={handleSaveProfile} className="ml-9 px-5 pb-4 space-y-3 max-w-sm">
                                        <div className="grid grid-cols-2 gap-3">
                                            <div>
                                                <label className="text-xs text-muted-foreground mb-1 block">First name</label>
                                                <Input value={profileForm.first_name} onChange={e => setProfileForm(f => ({ ...f, first_name: e.target.value }))} placeholder="First name" className="h-8 text-sm" />
                                            </div>
                                            <div>
                                                <label className="text-xs text-muted-foreground mb-1 block">Last name</label>
                                                <Input value={profileForm.last_name} onChange={e => setProfileForm(f => ({ ...f, last_name: e.target.value }))} placeholder="Last name" className="h-8 text-sm" />
                                            </div>
                                        </div>
                                        <div>
                                            <label className="text-xs text-muted-foreground mb-1 block">Display name</label>
                                            <Input value={profileForm.display_name} onChange={e => setProfileForm(f => ({ ...f, display_name: e.target.value }))} placeholder="Display name" className="h-8 text-sm" />
                                        </div>
                                        <ErrorMsg msg={profileError} />
                                        <div className="flex gap-2">
                                            <Button size="sm" type="submit" disabled={profileSaving}>
                                                <IconCheck size={14} /> {profileSaving ? 'Saving…' : 'Save'}
                                            </Button>
                                            <Button variant="ghost" size="sm" type="button" onClick={() => { setEditingProfile(false); setProfileError(''); }}>
                                                <IconX size={14} /> Cancel
                                            </Button>
                                        </div>
                                    </form>
                                </ExpandForm>
                            </FactorRow>
                        </div>
                    </motion.div>
                )}

                {/* Skeleton */}
                {isLoading && (
                    <div className="rounded-2xl border border-border bg-card">
                        {[...Array(5)].map((_, i) => (
                            <div key={i} className={`px-5 py-4 flex items-center gap-4 ${i < 4 ? 'border-b border-border/60' : ''}`}>
                                <div className="size-5 rounded bg-muted animate-pulse shrink-0" />
                                <div className="flex-1 space-y-2">
                                    <div className="h-3.5 w-28 rounded bg-muted animate-pulse" />
                                    <div className="h-3 w-40 rounded bg-muted animate-pulse" />
                                </div>
                            </div>
                        ))}
                    </div>
                )}

                {isError && (
                    <div className="rounded-2xl border border-border bg-card px-5 py-4">
                        <p className="text-sm text-destructive">Failed to load — make sure you are signed in.</p>
                    </div>
                )}

                {/* Security Factors Card */}
                {data && (
                    <motion.div initial={{ opacity: 0, y: 6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}>
                        <h2 className="text-sm font-medium text-muted-foreground mb-3">Security</h2>
                        <div className="rounded-2xl border border-border bg-card">
                            <PasswordRow isSet={data.password.is_set} onRefetch={refetch} />
                            <TotpRow totp={data.totp} onRefetch={refetch} />
                            <WebAuthnRow keys={data.webauthn} onRefetch={refetch} />
                            <PgpRow pgp={data.pgp} onRefetch={refetch} />
                            <RecoveryCodesRow remaining={data.recovery_codes.remaining_codes} onRefetch={refetch} />
                        </div>
                    </motion.div>
                )}

                {/* Danger Zone */}
                {data && (
                    <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ duration: 0.25, delay: 0.1 }}
                        className="mt-8">
                        <h2 className="text-sm font-medium text-destructive/80 mb-3">Danger Zone</h2>
                        <div className="rounded-2xl border border-destructive/20 bg-card">
                            <div className="px-5 py-4">
                                <div className="flex items-center justify-between">
                                    <div>
                                        <h3 className="text-sm font-medium">Delete account</h3>
                                        <p className="text-xs text-muted-foreground">Permanently remove your account and all associated data.</p>
                                    </div>
                                    {!showDelete && (
                                        <Button variant="destructive" size="sm" onClick={() => setShowDelete(true)}>
                                            <IconTrash size={14} /> Delete
                                        </Button>
                                    )}
                                </div>
                                <ExpandForm open={showDelete}>
                                    <form onSubmit={handleDeleteAccount} className="mt-1 space-y-3 max-w-sm pb-1">
                                        <p className="text-xs text-destructive">This action is irreversible. Enter your password to confirm.</p>
                                        <PasswordInput value={deletePassword} onChange={setDeletePassword} placeholder="Your password" required />
                                        <ErrorMsg msg={deleteError} />
                                        <div className="flex gap-2">
                                            <Button variant="destructive" size="sm" type="submit" disabled={deleting}>
                                                {deleting ? 'Deleting…' : 'Delete my account'}
                                            </Button>
                                            <Button variant="ghost" size="sm" type="button" onClick={() => { setShowDelete(false); setDeletePassword(''); setDeleteError(''); }}>
                                                Cancel
                                            </Button>
                                        </div>
                                    </form>
                                </ExpandForm>
                            </div>
                        </div>
                    </motion.div>
                )}
            </div>
        </div>
    );
}
