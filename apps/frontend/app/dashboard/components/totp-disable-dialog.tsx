'use client';

import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@components/ui/dialog';
import { Button } from '@components/ui/button';

interface TotpDisableDialogProps {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    displayName: string;
    isLoading: boolean;
    isError: boolean;
    onCancel: () => void;
    onConfirm: () => void;
}

export function TotpDisableDialog({
    open,
    onOpenChange,
    displayName,
    isLoading,
    isError,
    onCancel,
    onConfirm,
}: TotpDisableDialogProps) {
    return (
        <Dialog open={open} onOpenChange={onOpenChange}>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Remove authenticator</DialogTitle>
                    <DialogDescription>
                        This will remove{' '}
                        <span className="font-medium text-foreground">{displayName}</span> from your
                        account. You won&apos;t be able to use it for two-factor authentication
                        until you set up a new one.
                    </DialogDescription>
                </DialogHeader>
                {isError && (
                    <p className="text-xs text-destructive">Failed to remove authenticator.</p>
                )}
                <DialogFooter>
                    <Button variant="outline" onClick={onCancel} disabled={isLoading}>
                        Cancel
                    </Button>
                    <Button variant="destructive" onClick={onConfirm} disabled={isLoading}>
                        {isLoading ? 'Removing…' : 'Remove'}
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    );
}
