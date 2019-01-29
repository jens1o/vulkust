/// Node is reponsible to record command buffers.
/// It gathers all the needed data like pipeline, descriptor-sets, ... .
/// Its each node has input and output links.
/// There are some predifined ids for links, in addition users of engine can specify their ids
/// Node must bring a structure for its output link, and must accept a trait as its dependancy