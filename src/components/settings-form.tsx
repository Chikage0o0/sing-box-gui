import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { Button } from '@/components/ui/button'
import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { useEffect } from 'react'
import { formSchema, useSettingStore } from '@/stores/useSettingStore'
import { z } from 'zod'
import { Switch } from '@/components/ui/switch'

export function FormComponent() {
    const { settingData, setSetting } = useSettingStore()

    // 使用store中的值初始化表单
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: settingData,
    })

    // 当store中的值发生变化时，重置表单
    useEffect(() => {
        const isDirty = form.formState.isDirty
        if (!isDirty) {
            form.reset(settingData)
        }
    }, [settingData])

    // 提交时更新store
    function onSubmit(values: z.infer<typeof formSchema>) {
        setSetting(values)
        console.log('Form submitted, store updated:', values)
        // 这里可以添加其他提交后的逻辑
    }

    return (
        <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
                <FormField
                    control={form.control}
                    name="server.subscribe_url"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>订阅地址</FormLabel>
                            <FormControl>
                                <Input {...field} />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <div className="flex flex-row justify-between">
                    <FormField
                        control={form.control}
                        name="client.auto_start"
                        render={({ field }) => (
                            <FormItem className="flex items-center space-x-2 space-y-0">
                                <FormLabel>开机自启</FormLabel>
                                <FormControl>
                                    <Switch
                                        checked={field.value}
                                        onCheckedChange={field.onChange}
                                    />
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    <FormField
                        control={form.control}
                        name="client.silent_start"
                        render={({ field }) => (
                            <FormItem className="flex items-center space-x-2 space-y-0">
                                <FormLabel>静默启动</FormLabel>
                                <FormControl>
                                    <Switch
                                        checked={field.value}
                                        onCheckedChange={field.onChange}
                                    />
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                </div>
                <div className="flex flex-row justify-between">
                    <Button
                        type="reset"
                        variant="secondary"
                        onClick={() => form.reset()}>
                        重置
                    </Button>
                    <Button type="submit">提交</Button>
                </div>
            </form>
        </Form>
    )
}
