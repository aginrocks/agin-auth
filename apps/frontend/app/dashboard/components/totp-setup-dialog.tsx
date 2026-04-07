'use client';

import { UseFormReturn } from 'react-hook-form';
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
} from '@components/ui/dialog';
import { Form, FormControl, FormField, FormItem, FormMessage } from '@components/ui/form';
import { InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot } from '@components/ui/input-otp';
import { REGEXP_ONLY_DIGITS } from 'input-otp';
import QRCode from 'react-qr-code';
import { CopyButton } from './helpers';

type CodeForm = { code: string };

interface TotpSetupDialogProps {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    setupData: { secret: string; qr: string } | null;
    codeForm: UseFormReturn<CodeForm>;
    onSubmit: (data: CodeForm) => void;
}

export function TotpSetupDialog({
    open,
    onOpenChange,
    setupData,
    codeForm,
    onSubmit,
}: TotpSetupDialogProps) {
    return (
        <Dialog open={open} onOpenChange={onOpenChange}>
            <DialogContent className="sm:max-w-md">
                <DialogHeader>
                    <DialogTitle>Set up authenticator</DialogTitle>
                    <DialogDescription>
                        Scan the QR code with your authenticator app, then enter the verification
                        code.
                    </DialogDescription>
                </DialogHeader>
                {setupData && (
                    <div className="space-y-4">
                        <div className="flex justify-center">
                            <div className="p-3 bg-white rounded-lg">
                                <QRCode value={setupData.qr} size={160} />
                            </div>
                        </div>
                        <div className="rounded-lg border border-border bg-muted/30 p-3">
                            <div className="flex items-center justify-between mb-1">
                                <span className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
                                    Secret
                                </span>
                                <CopyButton text={setupData.secret} compact />
                            </div>
                            <code className="font-mono text-[11px] break-all text-foreground leading-relaxed">
                                {setupData.secret}
                            </code>
                        </div>

                        <Form {...codeForm}>
                            <form onSubmit={codeForm.handleSubmit(onSubmit)}>
                                <FormField
                                    control={codeForm.control}
                                    name="code"
                                    render={({ field }) => (
                                        <FormItem className="flex flex-col items-center gap-2">
                                            <FormControl>
                                                <InputOTP
                                                    maxLength={6}
                                                    pattern={REGEXP_ONLY_DIGITS}
                                                    {...field}
                                                >
                                                    <InputOTPGroup>
                                                        <InputOTPSlot index={0} />
                                                        <InputOTPSlot index={1} />
                                                        <InputOTPSlot index={2} />
                                                    </InputOTPGroup>
                                                    <InputOTPSeparator />
                                                    <InputOTPGroup>
                                                        <InputOTPSlot index={3} />
                                                        <InputOTPSlot index={4} />
                                                        <InputOTPSlot index={5} />
                                                    </InputOTPGroup>
                                                </InputOTP>
                                            </FormControl>
                                            <FormMessage />
                                        </FormItem>
                                    )}
                                />
                            </form>
                        </Form>
                    </div>
                )}
            </DialogContent>
        </Dialog>
    );
}
