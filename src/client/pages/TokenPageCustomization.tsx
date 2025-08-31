import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Badge } from '@/components/ui/badge';
import { 
    Upload, 
    Image, 
    Video, 
    FileText, 
    Link, 
    Palette,
    Share2,
    Settings,
    Eye,
    Download
} from 'lucide-react';

const TokenPageCustomization: React.FC = () => {
    const { publicKey } = useWallet();
    const [activeTab, setActiveTab] = useState('media');
    const [selectedTheme, setSelectedTheme] = useState('default');
    const [selectedLayout, setSelectedLayout] = useState('standard');
    const [primaryColor, setPrimaryColor] = useState('#3B82F6');
    const [secondaryColor, setSecondaryColor] = useState('#1F2937');
    const [backgroundColor, setBackgroundColor] = useState('#FFFFFF');
    const [textColor, setTextColor] = useState('#000000');
    const [accentColor, setAccentColor] = useState('#10B981');
    const [seoTitle, setSeoTitle] = useState('');
    const [seoDescription, setSeoDescription] = useState('');
    const [seoKeywords, setSeoKeywords] = useState('');
    const [uploadedFiles, setUploadedFiles] = useState<any[]>([]);
    const [socialLinks, setSocialLinks] = useState<any[]>([]);

    const themes = [
        { value: 'default', label: 'Default', preview: 'bg-gradient-to-r from-blue-500 to-purple-600' },
        { value: 'dark', label: 'Dark', preview: 'bg-gradient-to-r from-gray-800 to-gray-900' },
        { value: 'light', label: 'Light', preview: 'bg-gradient-to-r from-gray-100 to-gray-200' },
        { value: 'custom', label: 'Custom', preview: 'bg-gradient-to-r from-pink-500 to-yellow-500' },
    ];

    const layouts = [
        { value: 'standard', label: 'Standard', description: 'Classic layout with header, content, and sidebar' },
        { value: 'minimal', label: 'Minimal', description: 'Clean and simple design' },
        { value: 'gallery', label: 'Gallery', description: 'Focus on media and images' },
        { value: 'social', label: 'Social', description: 'Social media optimized layout' },
    ];

    const socialPlatforms = [
        { value: 'twitter', label: 'Twitter', icon: '🐦' },
        { value: 'discord', label: 'Discord', icon: '🎮' },
        { value: 'telegram', label: 'Telegram', icon: '📱' },
        { value: 'reddit', label: 'Reddit', icon: '🤖' },
        { value: 'instagram', label: 'Instagram', icon: '📷' },
        { value: 'tiktok', label: 'TikTok', icon: '🎵' },
        { value: 'youtube', label: 'YouTube', icon: '📺' },
        { value: 'website', label: 'Website', icon: '🌐' },
    ];

    const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
        const files = event.target.files;
        if (files) {
            const newFiles = Array.from(files).map((file, index) => ({
                id: Date.now() + index,
                name: file.name,
                size: file.size,
                type: file.type,
                url: URL.createObjectURL(file),
                uploadedAt: new Date(),
            }));
            setUploadedFiles([...uploadedFiles, ...newFiles]);
        }
    };

    const handleAddSocialLink = () => {
        setSocialLinks([...socialLinks, {
            id: Date.now(),
            platform: '',
            url: '',
            followerCount: 0,
        }]);
    };

    const handleUpdateSocialLink = (id: number, field: string, value: string | number) => {
        setSocialLinks(socialLinks.map(link => 
            link.id === id ? { ...link, [field]: value } : link
        ));
    };

    const handleRemoveSocialLink = (id: number) => {
        setSocialLinks(socialLinks.filter(link => link.id !== id));
    };

    const formatFileSize = (bytes: number): string => {
        if (bytes === 0) return '0 Bytes';
        const k = 1024;
        const sizes = ['Bytes', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    };

    return (
        <div className="container mx-auto px-4 py-8">
            <div className="mb-8">
                <h1 className="text-4xl font-bold mb-2">🎨 Token Page Customization</h1>
                <p className="text-muted-foreground">
                    Create stunning token pages with rich media and social integration
                </p>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
                {/* Main Content */}
                <div className="lg:col-span-2">
                    <Tabs value={activeTab} onValueChange={setActiveTab} className="space-y-6">
                        <TabsList className="grid w-full grid-cols-4">
                            <TabsTrigger value="media" className="flex items-center gap-2">
                                <Upload className="w-4 h-4" />
                                Media
                            </TabsTrigger>
                            <TabsTrigger value="design" className="flex items-center gap-2">
                                <Palette className="w-4 h-4" />
                                Design
                            </TabsTrigger>
                            <TabsTrigger value="social" className="flex items-center gap-2">
                                <Share2 className="w-4 h-4" />
                                Social
                            </TabsTrigger>
                            <TabsTrigger value="seo" className="flex items-center gap-2">
                                <Settings className="w-4 h-4" />
                                SEO
                            </TabsTrigger>
                        </TabsList>

                        <TabsContent value="media" className="space-y-6">
                            <Card>
                                <CardHeader>
                                    <CardTitle>Upload Media Files</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <div className="border-2 border-dashed border-gray-300 rounded-lg p-8 text-center">
                                        <Upload className="w-12 h-12 mx-auto text-gray-400 mb-4" />
                                        <p className="text-lg font-medium mb-2">Upload your media files</p>
                                        <p className="text-sm text-muted-foreground mb-4">
                                            Support for images, videos, GIFs, and documents (max 10MB each)
                                        </p>
                                        <Button onClick={() => document.getElementById('file-upload')?.click()}>
                                            Choose Files
                                        </Button>
                                        <input
                                            id="file-upload"
                                            type="file"
                                            multiple
                                            accept="image/*,video/*,.gif,.pdf,.doc,.docx"
                                            className="hidden"
                                            onChange={handleFileUpload}
                                        />
                                    </div>
                                </CardContent>
                            </Card>

                            {uploadedFiles.length > 0 && (
                                <Card>
                                    <CardHeader>
                                        <CardTitle>Uploaded Files</CardTitle>
                                    </CardHeader>
                                    <CardContent>
                                        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                            {uploadedFiles.map((file) => (
                                                <div key={file.id} className="border rounded-lg p-4">
                                                    <div className="flex items-center gap-3">
                                                        <div className="w-12 h-12 bg-gray-100 rounded-lg flex items-center justify-center">
                                                            {file.type.startsWith('image/') ? (
                                                                <Image className="w-6 h-6 text-gray-600" />
                                                            ) : file.type.startsWith('video/') ? (
                                                                <Video className="w-6 h-6 text-gray-600" />
                                                            ) : (
                                                                <FileText className="w-6 h-6 text-gray-600" />
                                                            )}
                                                        </div>
                                                        <div className="flex-1">
                                                            <p className="font-medium text-sm">{file.name}</p>
                                                            <p className="text-xs text-muted-foreground">
                                                                {formatFileSize(file.size)}
                                                            </p>
                                                        </div>
                                                        <div className="flex gap-2">
                                                            <Button size="sm" variant="outline">
                                                                <Eye className="w-4 h-4" />
                                                            </Button>
                                                            <Button size="sm" variant="outline">
                                                                <Download className="w-4 h-4" />
                                                            </Button>
                                                        </div>
                                                    </div>
                                                </div>
                                            ))}
                                        </div>
                                    </CardContent>
                                </Card>
                            )}
                        </TabsContent>

                        <TabsContent value="design" className="space-y-6">
                            <Card>
                                <CardHeader>
                                    <CardTitle>Theme Selection</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                                        {themes.map((theme) => (
                                            <div
                                                key={theme.value}
                                                className={`cursor-pointer border-2 rounded-lg p-4 text-center ${
                                                    selectedTheme === theme.value ? 'border-primary' : 'border-gray-200'
                                                }`}
                                                onClick={() => setSelectedTheme(theme.value)}
                                            >
                                                <div className={`w-full h-16 rounded mb-2 ${theme.preview}`}></div>
                                                <p className="text-sm font-medium">{theme.label}</p>
                                            </div>
                                        ))}
                                    </div>
                                </CardContent>
                            </Card>

                            <Card>
                                <CardHeader>
                                    <CardTitle>Layout Options</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <div className="space-y-4">
                                        {layouts.map((layout) => (
                                            <div
                                                key={layout.value}
                                                className={`cursor-pointer border-2 rounded-lg p-4 ${
                                                    selectedLayout === layout.value ? 'border-primary' : 'border-gray-200'
                                                }`}
                                                onClick={() => setSelectedLayout(layout.value)}
                                            >
                                                <div className="flex items-center justify-between">
                                                    <div>
                                                        <p className="font-medium">{layout.label}</p>
                                                        <p className="text-sm text-muted-foreground">
                                                            {layout.description}
                                                        </p>
                                                    </div>
                                                    <Badge variant={selectedLayout === layout.value ? 'default' : 'secondary'}>
                                                        {selectedLayout === layout.value ? 'Selected' : 'Select'}
                                                    </Badge>
                                                </div>
                                            </div>
                                        ))}
                                    </div>
                                </CardContent>
                            </Card>

                            <Card>
                                <CardHeader>
                                    <CardTitle>Color Scheme</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
                                        <div>
                                            <Label htmlFor="primary-color">Primary Color</Label>
                                            <div className="flex gap-2 mt-1">
                                                <Input
                                                    id="primary-color"
                                                    type="color"
                                                    value={primaryColor}
                                                    onChange={(e) => setPrimaryColor(e.target.value)}
                                                    className="w-16 h-10"
                                                />
                                                <Input
                                                    value={primaryColor}
                                                    onChange={(e) => setPrimaryColor(e.target.value)}
                                                    placeholder="#3B82F6"
                                                />
                                            </div>
                                        </div>
                                        
                                        <div>
                                            <Label htmlFor="secondary-color">Secondary Color</Label>
                                            <div className="flex gap-2 mt-1">
                                                <Input
                                                    id="secondary-color"
                                                    type="color"
                                                    value={secondaryColor}
                                                    onChange={(e) => setSecondaryColor(e.target.value)}
                                                    className="w-16 h-10"
                                                />
                                                <Input
                                                    value={secondaryColor}
                                                    onChange={(e) => setSecondaryColor(e.target.value)}
                                                    placeholder="#1F2937"
                                                />
                                            </div>
                                        </div>
                                        
                                        <div>
                                            <Label htmlFor="background-color">Background Color</Label>
                                            <div className="flex gap-2 mt-1">
                                                <Input
                                                    id="background-color"
                                                    type="color"
                                                    value={backgroundColor}
                                                    onChange={(e) => setBackgroundColor(e.target.value)}
                                                    className="w-16 h-10"
                                                />
                                                <Input
                                                    value={backgroundColor}
                                                    onChange={(e) => setBackgroundColor(e.target.value)}
                                                    placeholder="#FFFFFF"
                                                />
                                            </div>
                                        </div>
                                        
                                        <div>
                                            <Label htmlFor="text-color">Text Color</Label>
                                            <div className="flex gap-2 mt-1">
                                                <Input
                                                    id="text-color"
                                                    type="color"
                                                    value={textColor}
                                                    onChange={(e) => setTextColor(e.target.value)}
                                                    className="w-16 h-10"
                                                />
                                                <Input
                                                    value={textColor}
                                                    onChange={(e) => setTextColor(e.target.value)}
                                                    placeholder="#000000"
                                                />
                                            </div>
                                        </div>
                                        
                                        <div>
                                            <Label htmlFor="accent-color">Accent Color</Label>
                                            <div className="flex gap-2 mt-1">
                                                <Input
                                                    id="accent-color"
                                                    type="color"
                                                    value={accentColor}
                                                    onChange={(e) => setAccentColor(e.target.value)}
                                                    className="w-16 h-10"
                                                />
                                                <Input
                                                    value={accentColor}
                                                    onChange={(e) => setAccentColor(e.target.value)}
                                                    placeholder="#10B981"
                                                />
                                            </div>
                                        </div>
                                    </div>
                                </CardContent>
                            </Card>
                        </TabsContent>

                        <TabsContent value="social" className="space-y-6">
                            <Card>
                                <CardHeader>
                                    <CardTitle className="flex items-center justify-between">
                                        Social Media Links
                                        <Button onClick={handleAddSocialLink} size="sm">
                                            <Link className="w-4 h-4 mr-2" />
                                            Add Link
                                        </Button>
                                    </CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <div className="space-y-4">
                                        {socialLinks.map((link) => (
                                            <div key={link.id} className="border rounded-lg p-4">
                                                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                                                    <div>
                                                        <Label>Platform</Label>
                                                        <Select
                                                            value={link.platform}
                                                            onValueChange={(value) => handleUpdateSocialLink(link.id, 'platform', value)}
                                                        >
                                                            <SelectTrigger>
                                                                <SelectValue placeholder="Select platform" />
                                                            </SelectTrigger>
                                                            <SelectContent>
                                                                {socialPlatforms.map((platform) => (
                                                                    <SelectItem key={platform.value} value={platform.value}>
                                                                        <span className="flex items-center gap-2">
                                                                            {platform.icon} {platform.label}
                                                                        </span>
                                                                    </SelectItem>
                                                                ))}
                                                            </SelectContent>
                                                        </Select>
                                                    </div>
                                                    
                                                    <div>
                                                        <Label>URL</Label>
                                                        <Input
                                                            value={link.url}
                                                            onChange={(e) => handleUpdateSocialLink(link.id, 'url', e.target.value)}
                                                            placeholder="https://..."
                                                        />
                                                    </div>
                                                    
                                                    <div>
                                                        <Label>Follower Count</Label>
                                                        <Input
                                                            type="number"
                                                            value={link.followerCount}
                                                            onChange={(e) => handleUpdateSocialLink(link.id, 'followerCount', parseInt(e.target.value) || 0)}
                                                            placeholder="0"
                                                        />
                                                    </div>
                                                </div>
                                                
                                                <div className="flex justify-end mt-4">
                                                    <Button
                                                        variant="outline"
                                                        size="sm"
                                                        onClick={() => handleRemoveSocialLink(link.id)}
                                                    >
                                                        Remove
                                                    </Button>
                                                </div>
                                            </div>
                                        ))}
                                        
                                        {socialLinks.length === 0 && (
                                            <div className="text-center py-8 text-muted-foreground">
                                                <Link className="w-12 h-12 mx-auto mb-4" />
                                                <p>No social links added yet</p>
                                                <p className="text-sm">Add your social media profiles to increase visibility</p>
                                            </div>
                                        )}
                                    </div>
                                </CardContent>
                            </Card>
                        </TabsContent>

                        <TabsContent value="seo" className="space-y-6">
                            <Card>
                                <CardHeader>
                                    <CardTitle>SEO Settings</CardTitle>
                                </CardHeader>
                                <CardContent className="space-y-4">
                                    <div>
                                        <Label htmlFor="seo-title">Page Title</Label>
                                        <Input
                                            id="seo-title"
                                            value={seoTitle}
                                            onChange={(e) => setSeoTitle(e.target.value)}
                                            placeholder="Enter page title for search engines"
                                        />
                                    </div>
                                    
                                    <div>
                                        <Label htmlFor="seo-description">Meta Description</Label>
                                        <Textarea
                                            id="seo-description"
                                            value={seoDescription}
                                            onChange={(e) => setSeoDescription(e.target.value)}
                                            placeholder="Enter meta description for search engines"
                                            rows={3}
                                        />
                                    </div>
                                    
                                    <div>
                                        <Label htmlFor="seo-keywords">Keywords</Label>
                                        <Input
                                            id="seo-keywords"
                                            value={seoKeywords}
                                            onChange={(e) => setSeoKeywords(e.target.value)}
                                            placeholder="Enter keywords separated by commas"
                                        />
                                    </div>
                                </CardContent>
                            </Card>
                        </TabsContent>
                    </Tabs>
                </div>

                {/* Preview Sidebar */}
                <div className="lg:col-span-1">
                    <Card className="sticky top-8">
                        <CardHeader>
                            <CardTitle>Live Preview</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <div 
                                className="w-full h-64 rounded-lg border-2 border-dashed border-gray-300 flex items-center justify-center"
                                style={{
                                    backgroundColor: backgroundColor,
                                    color: textColor,
                                }}
                            >
                                <div className="text-center">
                                    <div className="text-2xl font-bold mb-2">Token Preview</div>
                                    <p className="text-sm opacity-75">
                                        Your customized token page will appear here
                                    </p>
                                </div>
                            </div>
                            
                            <div className="mt-4 space-y-2">
                                <div className="flex justify-between text-sm">
                                    <span>Theme:</span>
                                    <span className="font-medium">{selectedTheme}</span>
                                </div>
                                <div className="flex justify-between text-sm">
                                    <span>Layout:</span>
                                    <span className="font-medium">{selectedLayout}</span>
                                </div>
                                <div className="flex justify-between text-sm">
                                    <span>Files:</span>
                                    <span className="font-medium">{uploadedFiles.length}</span>
                                </div>
                                <div className="flex justify-between text-sm">
                                    <span>Social Links:</span>
                                    <span className="font-medium">{socialLinks.length}</span>
                                </div>
                            </div>
                            
                            <div className="mt-6 space-y-2">
                                <Button className="w-full">Save Changes</Button>
                                <Button variant="outline" className="w-full">Preview Full Page</Button>
                                <Button variant="outline" className="w-full">Publish</Button>
                            </div>
                        </CardContent>
                    </Card>
                </div>
            </div>
        </div>
    );
};

export default TokenPageCustomization;
