'use client';

import { useState, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { useQueryClient } from '@tanstack/react-query';
import { $api } from '@lib/providers/api';
import { useWebAuthnRegistration } from '@lib/hooks';
import { Button } from '@components/ui/button';
import { Input } from '@components/ui/input';
import { Label } from '@components/ui/label';
import {
    IconLock,
    IconDeviceMobile,
    IconFingerprint,
    IconKey,
    IconLifebuoy,
    IconTrash,
    IconCheck,
    IconCopy,
    IconEye,
    IconEyeOff,
    IconChevronRight,
} from '@tabler/icons-react';

// ── Helpers ────────────────────────────────────────────────────────────────────

function CopyButton({ text }: { text: string }) {
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

function ErrorMsg({ msg }: { msg: string }) {
    if (!msg) return null;
    return <p className="text-xs text-destructive mt-2">{msg}</p>;
}

function ExpandForm({ open, children }: { open: boolean; children: React.ReactNode }) {
    return (
        <AnimatePresence initial={false}>
            {open && (
                <motion.div
                    initial={{ opacity: 0, height: 0 }}
                    animate={{ opacity: 1, height: 'auto' }}
                    exit={{ opacity: 0, height: 0 }}
                    transition={{ type: 'spring', stiffness: 500, damping: 35, mass: 0.8 }}
                    className="overflow-hidden"
                >
                    <div className="pt-3 pb-1">{children}</div>
                </motion.div>
            )}
        </AnimatePresence>
    );
}

function SmoothResize({ children }: { children: React.ReactNode }) {
    const ref = useRef<HTMLDivElement>(null);
    const [height, setHeight] = useState<number | 'auto'>('auto');

    useEffect(() => {
        if (!ref.current) return;
        const ro = new ResizeObserver(([entry]) => setHeight(entry.contentRect.height));
        ro.observe(ref.current);
        return () => ro.disconnect();
    }, []);

    return (
        <motion.div animate={{ height }} transition={{ type: 'spring', stiffness: 500, damping: 35, mass: 0.8 }} className="overflow-hidden">
            <div ref={ref}>{children}</div>
        </motion.div>
    );
}

function PasswordInput({ value, onChange, placeholder, required, minLength, id }: {
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

// ── Row shell ─────────────────────────────────────────────────────────────────

function StatusTag({ label, enabled }: { label: string; enabled: boolean }) {
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

function FactorRow({
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

// ── Password ──────────────────────────────────────────────────────────────────

function PasswordRow({ isSet, onRefetch }: { isSet: boolean; onRefetch: () => void }) {
    const [open, setOpen] = useState(false);
    const [current, setCurrent] = useState('');
    const [next, setNext] = useState('');
    const [error, setError] = useState('');
    const change = $api.useMutation('post', '/api/settings/password/change');

    const submit = async (e: React.FormEvent) => {
        e.preventDefault();
        setError('');
        try {
            await change.mutateAsync({ body: { current_password: current, new_password: next } });
            setCurrent(''); setNext(''); setOpen(false); onRefetch();
        } catch { setError('Incorrect current password or invalid new password.'); }
    };

    return (
        <FactorRow icon={<IconLock />} name="Password" description="Authenticate using a knowledge-based password." tag={{ label: isSet ? 'Enabled' : 'Disabled', enabled: isSet }} onToggle={() => setOpen(v => !v)} open={open}>
            <ExpandForm open={open}>
                <form onSubmit={submit} className="space-y-3 max-w-sm ml-9 px-5 pb-4">
                    {isSet && (
                        <div className="space-y-1.5">
                            <Label htmlFor="current-password" className="text-xs">Current password</Label>
                            <PasswordInput id="current-password" value={current} onChange={setCurrent} required />
                        </div>
                    )}
                    <div className="space-y-1.5">
                        <Label htmlFor="new-password" className="text-xs">New password</Label>
                        <PasswordInput id="new-password" value={next} onChange={setNext} minLength={8} required placeholder="Min. 8 characters" />
                    </div>
                    <ErrorMsg msg={error} />
                    <div className="flex gap-2 pt-1">
                        <Button size="sm" type="submit" disabled={change.isPending}>
                            {change.isPending ? 'Saving…' : isSet ? 'Update' : 'Set password'}
                        </Button>
                        <Button size="sm" variant="ghost" type="button" onClick={() => { setOpen(false); setError(''); }}>
                            Cancel
                        </Button>
                    </div>
                </form>
            </ExpandForm>
        </FactorRow>
    );
}

// ── TOTP ──────────────────────────────────────────────────────────────────────

function TotpRow({ totp, onRefetch }: { totp: { display_name: string; fully_enabled: boolean } | null | undefined; onRefetch: () => void }) {
    const [step, setStep] = useState<'idle' | 'name' | 'confirm'>('idle');
    const [setupData, setSetupData] = useState<{ secret: string; qr: string } | null>(null);
    const [displayName, setDisplayName] = useState('');
    const [code, setCode] = useState('');
    const [error, setError] = useState('');

    const enable = $api.useMutation('post', '/api/settings/factors/totp/enable');
    const confirm = $api.useMutation('post', '/api/settings/factors/totp/enable/confirm');
    const disable = $api.useMutation('delete', '/api/settings/factors/totp/disable');

    const isEnabled = totp?.fully_enabled ?? false;

    const startSetup = async (e: React.FormEvent) => {
        e.preventDefault(); setError('');
        try { const d = await enable.mutateAsync({ body: { display_name: displayName } }); setSetupData(d); setStep('confirm'); }
        catch { setError('Failed to start setup.'); }
    };

    const confirmSetup = async (e: React.FormEvent) => {
        e.preventDefault(); setError('');
        try { await confirm.mutateAsync({ body: { code } }); setStep('idle'); setSetupData(null); setCode(''); setDisplayName(''); onRefetch(); }
        catch { setError('Invalid code, try again.'); }
    };

    const handleDisable = async () => {
        setError('');
        try { await disable.mutateAsync({}); onRefetch(); }
        catch { setError('Failed to disable.'); }
    };


    const handleToggle = () => {
        if (step !== 'idle') { setStep('idle'); setSetupData(null); setError(''); }
        else if (isEnabled) { setStep('idle'); }
        else { setStep('name'); }
    };

    return (
        <FactorRow icon={<IconDeviceMobile />} name="Authenticator App" description="Time-based one-time passwords from your authenticator app." tag={{ label: isEnabled ? totp?.display_name ?? 'Enabled' : 'Disabled', enabled: isEnabled }} onToggle={handleToggle} open={step !== 'idle'}>
            <div className="ml-9 px-5">
                {step === 'idle' && isEnabled && (
                    <>
                        <ErrorMsg msg={error} />
                        <button onClick={handleDisable} disabled={disable.isPending}
                            className="mt-2 pb-3 flex items-center gap-1 text-xs text-destructive/60 hover:text-destructive transition-colors disabled:opacity-50">
                            {disable.isPending ? 'Disabling…' : 'Disable'}
                        </button>
                    </>
                )}

                <ExpandForm open={step === 'name'}>
                    <form onSubmit={startSetup} className="space-y-3 max-w-sm pb-4">
                        <div className="space-y-1.5">
                            <Label htmlFor="totp-name" className="text-xs">Authenticator name</Label>
                            <Input id="totp-name" value={displayName} onChange={e => setDisplayName(e.target.value)}
                                placeholder="Authy, Google Authenticator…" className="h-9 text-sm" required maxLength={32} />
                        </div>
                        <ErrorMsg msg={error} />
                        <div className="flex gap-2">
                            <Button size="sm" type="submit" disabled={enable.isPending}>
                                {enable.isPending ? 'Generating…' : 'Continue'}
                            </Button>
                            <Button size="sm" variant="ghost" type="button" onClick={() => { setStep('idle'); setError(''); }}>Cancel</Button>
                        </div>
                    </form>
                </ExpandForm>

                <ExpandForm open={step === 'confirm' && !!setupData}>
                    {setupData && (
                        <div className="space-y-3 max-w-sm pb-4">
                            <div className="rounded-lg border border-border bg-muted/40 p-3 space-y-2.5">
                                <p className="text-xs text-muted-foreground">Add to your authenticator app, then enter the code below.</p>
                                <div>
                                    <div className="flex items-center justify-between mb-1">
                                        <span className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">Secret</span>
                                        <CopyButton text={setupData.secret} />
                                    </div>
                                    <code className="font-mono text-xs break-all text-foreground">{setupData.secret}</code>
                                </div>
                                <div>
                                    <div className="flex items-center justify-between mb-1">
                                        <span className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">OTP URL</span>
                                        <CopyButton text={setupData.qr} />
                                    </div>
                                    <code className="font-mono text-[10px] break-all text-muted-foreground">{setupData.qr}</code>
                                </div>
                            </div>
                            <form onSubmit={confirmSetup} className="space-y-3">
                                <div className="space-y-1.5">
                                    <Label htmlFor="totp-code" className="text-xs">Verification code</Label>
                                    <Input id="totp-code" value={code} onChange={e => setCode(e.target.value.replace(/\D/g, '').slice(0, 6))}
                                        placeholder="000000" className="h-9 font-mono tracking-[0.3em] text-sm text-center" maxLength={6} required />
                                </div>
                                <ErrorMsg msg={error} />
                                <div className="flex gap-2">
                                    <Button size="sm" type="submit" disabled={confirm.isPending}>
                                        <IconCheck size={13} /> {confirm.isPending ? 'Verifying…' : 'Confirm'}
                                    </Button>
                                    <Button size="sm" variant="ghost" type="button" onClick={() => { setStep('idle'); setSetupData(null); setError(''); }}>Cancel</Button>
                                </div>
                            </form>
                        </div>
                    )}
                </ExpandForm>
            </div>
        </FactorRow>
    );
}

// ── WebAuthn ──────────────────────────────────────────────────────────────────

function WebAuthnRow({ keys, onRefetch }: { keys: { display_name: string }[]; onRefetch: () => void }) {
    const [adding, setAdding] = useState(false);
    const [newName, setNewName] = useState('');
    const [error, setError] = useState('');
    const [deletingKey, setDeletingKey] = useState<string | null>(null);

    const webAuthn = useWebAuthnRegistration();
    const deleteKey = $api.useMutation('delete', '/api/settings/factors/webauthn/delete/{display_name}');

    const handleAdd = async (e: React.FormEvent) => {
        e.preventDefault(); setError('');
        try { await webAuthn.registerAsync(newName); setNewName(''); setAdding(false); onRefetch(); }
        catch { setError('Failed to register passkey.'); }
    };

    const handleDelete = async (name: string) => {
        setDeletingKey(name);
        try { await deleteKey.mutateAsync({ params: { path: { display_name: name } } }); onRefetch(); }
        catch { setError('Failed to delete.'); }
        finally { setDeletingKey(null); }
    };


    return (
        <FactorRow icon={<IconFingerprint />} name="Passkeys" description="Phishing-resistant authentication using your device or a hardware security key." tag={{ label: keys.length > 0 ? `${keys.length} key${keys.length > 1 ? 's' : ''}` : 'Disabled', enabled: keys.length > 0 }} onToggle={() => setAdding(v => !v)} open={adding}>
            <div className="ml-9 px-5">
                {keys.length > 0 && (
                    <div className="space-y-1 mt-2 mb-3 max-w-sm">
                        {keys.map(key => (
                            <div key={key.display_name} className="flex items-center justify-between rounded-lg border border-border px-3 py-2 bg-muted/30">
                                <div className="flex items-center gap-2">
                                    <IconKey size={12} className="text-muted-foreground" />
                                    <span className="text-sm">{key.display_name}</span>
                                </div>
                                <button onClick={() => handleDelete(key.display_name)} disabled={deletingKey === key.display_name}
                                    aria-label={`Delete ${key.display_name}`}
                                    className="text-muted-foreground hover:text-destructive transition-colors disabled:opacity-30">
                                    <IconTrash size={13} />
                                </button>
                            </div>
                        ))}
                    </div>
                )}
                <ErrorMsg msg={error} />
                <ExpandForm open={adding}>
                    <form onSubmit={handleAdd} className="space-y-3 max-w-sm pb-4">
                        <div className="space-y-1.5">
                            <Label htmlFor="webauthn-name" className="text-xs">Key name</Label>
                            <Input id="webauthn-name" value={newName} onChange={e => setNewName(e.target.value)}
                                placeholder="YubiKey 5, iPhone Face ID…" className="h-9 text-sm" required maxLength={32} />
                        </div>
                        <div className="flex gap-2">
                            <Button size="sm" type="submit">Register</Button>
                            <Button size="sm" variant="ghost" type="button" onClick={() => { setAdding(false); setError(''); }}>Cancel</Button>
                        </div>
                    </form>
                </ExpandForm>
            </div>
        </FactorRow>
    );
}

// ── PGP ───────────────────────────────────────────────────────────────────────

function PgpRow({ pgp, onRefetch }: { pgp: { fingerprint: string; display_name: string }[]; onRefetch: () => void }) {
    const [open, setOpen] = useState(false);
    const [displayName, setDisplayName] = useState('');
    const [publicKey, setPublicKey] = useState('');
    const [error, setError] = useState('');

    const enable = $api.useMutation('post', '/api/settings/factors/pgp/enable');
    const disable = $api.useMutation('delete', '/api/settings/factors/pgp/disable');

    const isEnabled = pgp.length > 0;

    const handleEnable = async (e: React.FormEvent) => {
        e.preventDefault(); setError('');
        try { await enable.mutateAsync({ body: { display_name: displayName, public_key: publicKey } }); setOpen(false); setDisplayName(''); setPublicKey(''); onRefetch(); }
        catch { setError('Invalid key or failed to add.'); }
    };

    const handleDisable = async () => {
        setError('');
        try { await disable.mutateAsync({}); onRefetch(); }
        catch { setError('Failed to remove.'); }
    };


    return (
        <FactorRow icon={<IconKey />} name="PGP Key" description="Authenticate by signing a server challenge with your PGP private key." tag={{ label: isEnabled ? pgp[0]?.display_name ?? 'Enabled' : 'Disabled', enabled: isEnabled }} onToggle={() => { if (!isEnabled) setOpen(v => !v); }} open={open}>
            <div className="ml-9 px-5">
                {isEnabled ? (
                    <div className="space-y-2 mt-2 pb-3">
                        <div className="rounded-lg border border-border bg-muted/30 px-3 py-2 max-w-sm">
                            <p className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground mb-1">Fingerprint</p>
                            <code className="font-mono text-[11px] text-foreground break-all">{pgp[0]?.fingerprint}</code>
                        </div>
                        <ErrorMsg msg={error} />
                        <button onClick={handleDisable} disabled={disable.isPending}
                            className="flex items-center gap-1 text-xs text-destructive/60 hover:text-destructive transition-colors disabled:opacity-50">
                            {disable.isPending ? 'Removing…' : 'Remove key'}
                        </button>
                    </div>
                ) : (
                    <ExpandForm open={open}>
                        <form onSubmit={handleEnable} className="space-y-3 max-w-sm pb-4">
                            <div className="space-y-1.5">
                                <Label htmlFor="pgp-name" className="text-xs">Name</Label>
                                <Input id="pgp-name" value={displayName} onChange={e => setDisplayName(e.target.value)}
                                    placeholder="Work key" className="h-9 text-sm" required maxLength={32} />
                            </div>
                            <div className="space-y-1.5">
                                <Label className="text-xs">Public key (ASCII-armored)</Label>
                                <textarea value={publicKey} onChange={e => setPublicKey(e.target.value)}
                                    placeholder="-----BEGIN PGP PUBLIC KEY BLOCK-----"
                                    className="w-full rounded-md border border-input bg-background px-3 py-2 font-mono text-[11px] placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring/50 min-h-[80px] resize-none"
                                    required />
                            </div>
                            <ErrorMsg msg={error} />
                            <div className="flex gap-2">
                                <Button size="sm" type="submit" disabled={enable.isPending}>
                                    {enable.isPending ? 'Adding…' : 'Add key'}
                                </Button>
                                <Button size="sm" variant="ghost" type="button" onClick={() => { setOpen(false); setError(''); }}>Cancel</Button>
                            </div>
                        </form>
                    </ExpandForm>
                )}
            </div>
        </FactorRow>
    );
}

// ── Recovery Codes ────────────────────────────────────────────────────────────

function RecoveryCodesRow({ remaining, onRefetch }: { remaining: number; onRefetch: () => void }) {
    const [open, setOpen] = useState(false);
    const [codes, setCodes] = useState<string[] | null>(null);
    const [error, setError] = useState('');

    const enable = $api.useMutation('post', '/api/settings/factors/recovery-codes/enable');
    const reset = $api.useMutation('post', '/api/settings/factors/recovery-codes/reset');

    const isEnabled = remaining > 0;

    const handleEnable = async () => {
        setError('');
        try { const d = await enable.mutateAsync({}); setCodes(d.codes); onRefetch(); }
        catch { setError('Failed to generate codes.'); }
    };

    const handleReset = async () => {
        setError('');
        try { const d = await reset.mutateAsync({}); setCodes(d.codes); onRefetch(); }
        catch { setError('Failed to regenerate.'); }
    };

    return (
        <FactorRow icon={<IconLifebuoy />} name="Recovery Codes" description="One-time emergency codes to regain access if you lose your other factors." tag={{ label: isEnabled ? `${remaining} remaining` : 'Disabled', enabled: isEnabled }} onToggle={() => { setOpen(v => !v); setCodes(null); setError(''); }} open={open} last>
            <ExpandForm open={open}>
                <div className="ml-9 px-5 pb-4 max-w-md">
                    <SmoothResize>
                        {codes ? (
                            <div className="space-y-3">
                                <div className="rounded-xl border border-border bg-card p-4">
                                    <div className="flex items-center justify-between mb-3">
                                        <p className="text-xs font-medium text-foreground">Your recovery codes</p>
                                        <CopyButton text={codes.join('\n')} />
                                    </div>
                                    <div className="grid grid-cols-2 gap-x-8 gap-y-2 px-1">
                                        {codes.map((c, i) => (
                                            <div key={c} className="flex items-center gap-2.5">
                                                <span className="text-[10px] tabular-nums text-muted-foreground/60 w-3 text-right">{i + 1}</span>
                                                <code className="font-mono text-[13px] tracking-wide text-foreground">{c}</code>
                                            </div>
                                        ))}
                                    </div>
                                </div>
                                <p className="text-[11px] text-muted-foreground leading-relaxed">Store these codes in a safe place. Each code can only be used once.</p>
                                <Button size="sm" onClick={() => setCodes(null)}>
                                    <IconCheck size={13} /> Done, I saved them
                                </Button>
                            </div>
                        ) : (
                            <div className="space-y-3">
                                <p className="text-xs text-muted-foreground leading-relaxed">
                                    {isEnabled
                                        ? `${remaining} code${remaining !== 1 ? 's' : ''} remaining. Regenerating will replace all current codes.`
                                        : 'Generate one-time codes to access your account if you lose your other sign-in methods.'}
                                </p>
                                <ErrorMsg msg={error} />
                                <div className="flex gap-2">
                                    <Button size="sm" onClick={isEnabled ? handleReset : handleEnable} disabled={enable.isPending || reset.isPending}>
                                        {enable.isPending || reset.isPending ? 'Generating…' : isEnabled ? 'Regenerate' : 'Generate'}
                                    </Button>
                                    <Button size="sm" variant="ghost" onClick={() => { setOpen(false); setError(''); }}>Cancel</Button>
                                </div>
                            </div>
                        )}
                    </SmoothResize>
                </div>
            </ExpandForm>
        </FactorRow>
    );
}

// ── Dashboard ─────────────────────────────────────────────────────────────────

export default function DashboardPage() {
    const queryClient = useQueryClient();
    const { data, isLoading, isError } = $api.useQuery('get', '/api/settings/factors');

    const refetch = () =>
        queryClient.invalidateQueries({ queryKey: ['get', '/api/settings/factors'] });

    return (
        <div className="min-h-screen bg-background">
            <div className="mx-auto max-w-2xl px-6 py-14">

                {/* Header */}
                <motion.div initial={{ opacity: 0, y: -6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}
                    className="mb-8">
                    <h1 className="text-xl font-semibold mb-0.5">Account Security</h1>
                    <p className="text-sm text-muted-foreground">Keep your account info up to date to ensure you always have access.</p>
                </motion.div>

                {/* Skeleton */}
                {isLoading && (
                    <div className="rounded-2xl border border-border bg-card overflow-hidden">
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

                {/* Card */}
                {data && (
                    <motion.div initial={{ opacity: 0, y: 6 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25 }}
                        className="rounded-2xl border border-border bg-card overflow-hidden">
                        <PasswordRow isSet={data.password.is_set} onRefetch={refetch} />
                        <TotpRow totp={data.totp} onRefetch={refetch} />
                        <WebAuthnRow keys={data.webauthn} onRefetch={refetch} />
                        <PgpRow pgp={data.pgp} onRefetch={refetch} />
                        <RecoveryCodesRow remaining={data.recovery_codes.remaining_codes} onRefetch={refetch} />
                    </motion.div>
                )}
            </div>
        </div>
    );
}
