import { Button } from '@components/ui/button';

export function FactorKeyItem({ icon, name, subtitle, onRemove }: {
    icon: React.ReactNode;
    name: string;
    subtitle: string;
    onRemove: () => void;
}) {
    return (
        <div className="flex items-center justify-between rounded-lg border border-border/60 bg-muted/20 p-3">
            <div className="flex items-center gap-3">
                <div className="rounded-md bg-muted/60 p-1.5">
                    {icon}
                </div>
                <div>
                    <p className="text-sm font-medium leading-tight">{name}</p>
                    <p className="text-[10px] text-muted-foreground mt-0.5 break-all">{subtitle}</p>
                </div>
            </div>
            <Button variant="ghost" size="sm" onClick={onRemove}
                className="text-muted-foreground hover:text-destructive hover:bg-destructive/5 h-7 px-2 text-xs shrink-0 ml-3">
                Remove
            </Button>
        </div>
    );
}
