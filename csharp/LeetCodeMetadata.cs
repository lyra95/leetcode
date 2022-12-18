namespace csharp;

[AttributeUsage(AttributeTargets.Class)]
internal class LeetCodeMetadataAttribute : Attribute  
{
    public uint Number { get; init; }
    public string Title { get; init; }
    public string Url { get; init; }
}
