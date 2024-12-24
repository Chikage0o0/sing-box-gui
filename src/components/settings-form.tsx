import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'

export function LoginForm({
    className,
    ...props
}: React.ComponentPropsWithoutRef<'div'>) {
    return (
        <div className={cn('flex flex-col gap-6', className)} {...props}>
            <form>
                <div className="flex flex-col gap-6">
                    <div className="grid gap-2">
                        <Label htmlFor="subscribe">订阅地址</Label>
                        <Input
                            id="subscribe"
                            type="url"
                            placeholder="https://example.com/sing-box.json"
                            required
                        />
                    </div>
                    <Button type="submit" className="w-full">
                        保存
                    </Button>
                </div>
            </form>
        </div>
    )
}
