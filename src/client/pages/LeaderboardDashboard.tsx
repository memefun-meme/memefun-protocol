import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Progress } from '@/components/ui/progress';
import { Trophy, TrendingUp, Users, Target } from 'lucide-react';

const LeaderboardDashboard: React.FC = () => {
    const { publicKey } = useWallet();
    const [activeTab, setActiveTab] = useState('creators');
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        // Simulate loading
        setTimeout(() => setLoading(false), 1000);
    }, []);

    const formatNumber = (num: number): string => {
        if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
        if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
        return num.toString();
    };

    const getRankBadge = (rank: number) => {
        if (rank === 1) return <Badge className="bg-yellow-500">🥇 1st</Badge>;
        if (rank === 2) return <Badge className="bg-gray-400">🥈 2nd</Badge>;
        if (rank === 3) return <Badge className="bg-orange-600">🥉 3rd</Badge>;
        return <Badge variant="secondary">#{rank}</Badge>;
    };

    if (loading) {
        return (
            <div className="flex items-center justify-center min-h-screen">
                <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-primary"></div>
            </div>
        );
    }

    return (
        <div className="container mx-auto px-4 py-8">
            <div className="mb-8">
                <h1 className="text-4xl font-bold mb-2">🏆 Leaderboards</h1>
                <p className="text-muted-foreground">
                    Discover the top creators, traders, and tokens on the platform
                </p>
            </div>

            <Tabs value={activeTab} onValueChange={setActiveTab} className="space-y-6">
                <TabsList className="grid w-full grid-cols-3">
                    <TabsTrigger value="creators" className="flex items-center gap-2">
                        <Users className="w-4 h-4" />
                        Top Creators
                    </TabsTrigger>
                    <TabsTrigger value="traders" className="flex items-center gap-2">
                        <TrendingUp className="w-4 h-4" />
                        Top Traders
                    </TabsTrigger>
                    <TabsTrigger value="tokens" className="flex items-center gap-2">
                        <Target className="w-4 h-4" />
                        Top Tokens
                    </TabsTrigger>
                </TabsList>

                <TabsContent value="creators" className="space-y-6">
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {[1, 2, 3, 4, 5, 6].map((rank) => (
                            <Card key={rank} className="hover:shadow-lg transition-shadow">
                                <CardHeader className="pb-3">
                                    <div className="flex items-center justify-between">
                                        <div className="flex items-center gap-3">
                                            <Avatar>
                                                <AvatarImage src={`https://api.dicebear.com/7.x/avataaars/svg?seed=Creator${rank}`} />
                                                <AvatarFallback>CR</AvatarFallback>
                                            </Avatar>
                                            <div>
                                                <CardTitle className="text-lg">Creator {rank}</CardTitle>
                                                <p className="text-sm text-muted-foreground">
                                                    {5 + rank} tokens created
                                                </p>
                                            </div>
                                        </div>
                                        {getRankBadge(rank)}
                                    </div>
                                </CardHeader>
                                <CardContent className="space-y-4">
                                    <div className="grid grid-cols-2 gap-4 text-sm">
                                        <div>
                                            <p className="text-muted-foreground">Volume</p>
                                            <p className="font-semibold">{formatNumber(5000000 - rank * 500000)}</p>
                                        </div>
                                        <div>
                                            <p className="text-muted-foreground">Holders</p>
                                            <p className="font-semibold">{formatNumber(15000 - rank * 1000)}</p>
                                        </div>
                                        <div>
                                            <p className="text-muted-foreground">Success Rate</p>
                                            <p className="font-semibold">{(85 - rank * 5).toFixed(1)}%</p>
                                        </div>
                                        <div>
                                            <p className="text-muted-foreground">Revenue</p>
                                            <p className="font-semibold">{formatNumber(60000 - rank * 5000)}</p>
                                        </div>
                                    </div>
                                </CardContent>
                            </Card>
                        ))}
                    </div>
                </TabsContent>

                <TabsContent value="traders" className="space-y-6">
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {[1, 2, 3, 4, 5, 6].map((rank) => (
                            <Card key={rank} className="hover:shadow-lg transition-shadow">
                                <CardHeader className="pb-3">
                                    <div className="flex items-center justify-between">
                                        <div className="flex items-center gap-3">
                                            <Avatar>
                                                <AvatarImage src={`https://api.dicebear.com/7.x/avataaars/svg?seed=Trader${rank}`} />
                                                <AvatarFallback>TR</AvatarFallback>
                                            </Avatar>
                                            <div>
                                                <CardTitle className="text-lg">Trader {rank}</CardTitle>
                                                <p className="text-sm text-muted-foreground">
                                                    {150 - rank * 10} trades
                                                </p>
                                            </div>
                                        </div>
                                        {getRankBadge(rank)}
                                    </div>
                                </CardHeader>
                                <CardContent className="space-y-4">
                                    <div className="grid grid-cols-2 gap-4 text-sm">
                                        <div>
                                            <p className="text-muted-foreground">Total ROI</p>
                                            <p className="font-semibold text-green-600">
                                                +{(245 - rank * 20).toFixed(1)}%
                                            </p>
                                        </div>
                                        <div>
                                            <p className="text-muted-foreground">Volume</p>
                                            <p className="font-semibold">{formatNumber(10000000 - rank * 500000)}</p>
                                        </div>
                                        <div>
                                            <p className="text-muted-foreground">Profit</p>
                                            <p className="font-semibold text-green-600">
                                                {formatNumber(2450000 - rank * 200000)}
                                            </p>
                                        </div>
                                        <div>
                                            <p className="text-muted-foreground">Accuracy</p>
                                            <p className="font-semibold">{(82 - rank * 3).toFixed(1)}%</p>
                                        </div>
                                    </div>
                                </CardContent>
                            </Card>
                        ))}
                    </div>
                </TabsContent>

                <TabsContent value="tokens" className="space-y-6">
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {[1, 2, 3, 4, 5, 6].map((rank) => (
                            <Card key={rank} className="hover:shadow-lg transition-shadow">
                                <CardHeader className="pb-3">
                                    <div className="flex items-center justify-between">
                                        <div className="flex items-center gap-3">
                                            <Avatar>
                                                <AvatarImage src={`https://api.dicebear.com/7.x/avataaars/svg?seed=Token${rank}`} />
                                                <AvatarFallback>TK</AvatarFallback>
                                            </Avatar>
                                            <div>
                                                <CardTitle className="text-lg">Token {rank}</CardTitle>
                                                <p className="text-sm text-muted-foreground">
                                                    by Creator {rank}
                                                </p>
                                            </div>
                                        </div>
                                        {getRankBadge(rank)}
                                    </div>
                                </CardHeader>
                                <CardContent className="space-y-4">
                                    <div className="grid grid-cols-2 gap-4 text-sm">
                                        <div>
                                            <p className="text-muted-foreground">Market Cap</p>
                                            <p className="font-semibold">{formatNumber(50000000 - rank * 3000000)}</p>
                                        </div>
                                        <div>
                                            <p className="text-muted-foreground">24h Volume</p>
                                            <p className="font-semibold">{formatNumber(2500000 - rank * 200000)}</p>
                                        </div>
                                        <div>
                                            <p className="text-muted-foreground">Growth</p>
                                            <p className="font-semibold text-green-600">
                                                +{(15.5 - rank * 1.5).toFixed(1)}%
                                            </p>
                                        </div>
                                        <div>
                                            <p className="text-muted-foreground">Holders</p>
                                            <p className="font-semibold">{formatNumber(8500 - rank * 500)}</p>
                                        </div>
                                    </div>
                                </CardContent>
                            </Card>
                        ))}
                    </div>
                </TabsContent>
            </Tabs>
        </div>
    );
};

export default LeaderboardDashboard;
